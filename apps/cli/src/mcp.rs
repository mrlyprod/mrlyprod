use mrlycore::json::Map;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use std::collections::HashMap;
use std::io::{BufRead, Write};

const VERSIONS: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];
const LATEST: &str = "2025-06-18";
const READ_TOOL: &str = "mrly_read";
const INSTRUCTIONS: &str =
    "mrly is a deterministic world: every tool call is journaled and replayable. \
The surface is live, so tools appear and vanish as state changes. \
Reads are addresses: mrly_read with a path like chess/board returns one small subtree, \
while a bare app name returns its whole view. Prefer the deepest address that answers.";

pub fn serve() {
    let mut os = mrlyweb::registry::boot("full");
    let mut names: HashMap<String, String> = HashMap::new();
    let mut rendered = tools(&os, &mut names).to_string();
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let message = match mrlycore::json::parse(&line) {
            Ok(m) => m,
            Err(note) => {
                emit(&fail(&Json::Null, -32700, &format!("{note}")));
                continue;
            }
        };
        if message.as_array().is_some() {
            emit(&fail(
                &Json::Null,
                -32600,
                "batching left the protocol in 2025-06-18",
            ));
            continue;
        }
        let id = message["id"].clone();
        let method = message["method"].as_str().unwrap_or("").to_string();
        let params = message["params"].clone();
        if message["id"].is_null() && !message.as_object().is_some_and(|m| m.contains_key("id")) {
            continue;
        }
        let answer = match method.as_str() {
            "initialize" => reply(&id, initialize(&params)),
            "ping" => reply(&id, json!({})),
            "tools/list" => reply(&id, json!({ "tools": tools(&os, &mut names) })),
            "tools/call" => match invoke(&mut os, &params, &names) {
                Ok(result) => {
                    let now = tools(&os, &mut names).to_string();
                    let moved = now != rendered;
                    rendered = now;
                    let out = reply(&id, result);
                    emit(&out);
                    if moved {
                        emit(
                            &json!({ "jsonrpc": "2.0", "method": "notifications/tools/list_changed" }),
                        );
                    }
                    continue;
                }
                Err(note) => fail(&id, -32602, &note),
            },
            "resources/list" => reply(&id, resources(&os)),
            "resources/templates/list" => reply(
                &id,
                json!({ "resourceTemplates": [{
                    "uriTemplate": "mrly://read/{app}/{+path}",
                    "name": "subtree",
                    "description": "one drilled fact of an app's state",
                    "mimeType": "application/json",
                }] }),
            ),
            "resources/read" => match fetch(&os, &params) {
                Ok(result) => reply(&id, result),
                Err(note) => fail(&id, -32602, &note),
            },
            _ => fail(&id, -32601, &format!("method not found: {method}")),
        };
        emit(&answer);
    }
}

fn emit(message: &Json) {
    let mut out = std::io::stdout().lock();
    let _ = writeln!(out, "{message}");
    let _ = out.flush();
}

fn reply(id: &Json, result: Json) -> Json {
    json!({ "jsonrpc": "2.0", "id": id.clone(), "result": result })
}

fn fail(id: &Json, code: i64, message: &str) -> Json {
    json!({ "jsonrpc": "2.0", "id": id.clone(), "error": { "code": code, "message": message } })
}

fn initialize(params: &Json) -> Json {
    let asked = params["protocolVersion"].as_str().unwrap_or("");
    let version = if VERSIONS.contains(&asked) {
        asked
    } else {
        LATEST
    };
    json!({
        "protocolVersion": version,
        "capabilities": {
            "tools": { "listChanged": true },
            "resources": { "subscribe": false, "listChanged": false },
        },
        "serverInfo": { "name": "mrly", "version": mrlyos::kernel::VERSION },
        "instructions": INSTRUCTIONS,
    })
}

fn tools(os: &Os, names: &mut HashMap<String, String>) -> Json {
    names.clear();
    let surface = os.list(None);
    let mut out: Vec<Json> = Vec::new();
    let mut verbs: Vec<Json> = surface["nav"].as_array().cloned().unwrap_or_default();
    for app in surface["verbs"].as_array().cloned().unwrap_or_default() {
        verbs.extend(app["verbs"].as_array().cloned().unwrap_or_default());
    }
    for verb in verbs {
        let Some(dotted) = verb["verb"].as_str() else {
            continue;
        };
        let name = dotted.replace('.', "_");
        names.insert(name.clone(), dotted.to_string());
        out.push(json!({
            "name": name,
            "description": dotted,
            "inputSchema": schema(&verb["args"]),
        }));
    }
    out.push(json!({
        "name": READ_TOOL,
        "description": "read the world: \"\" for the envelope, \"chess\" for a view, \"chess/board\" for a subtree",
        "inputSchema": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "the read address" },
                "shape": { "type": "object", "description": "optional prune shape" },
            },
        },
    }));
    Json::Arr(out)
}

fn schema(args: &Json) -> Json {
    let mut properties = Map::new();
    if let Some(fields) = args.as_object() {
        for (key, hint) in fields {
            properties.insert(key.clone(), field(hint));
        }
    }
    properties.insert(
        "now".to_string(),
        json!({ "type": "integer", "description": "epoch millis; omit for wall clock" }),
    );
    json!({ "type": "object", "properties": Json::Obj(properties) })
}

fn field(hint: &Json) -> Json {
    let Some(token) = hint.as_str() else {
        return json!({ "description": hint.to_string() });
    };
    if token.contains(" | ") {
        let options: Vec<&str> = token.split(" | ").map(str::trim).collect();
        return json!({ "type": "string", "enum": options, "description": token });
    }
    if let Some(range) = token.strip_prefix("int ") {
        if let Some((lo, hi)) = range.split_once("..") {
            if let (Ok(lo), Ok(hi)) = (lo.trim().parse::<i64>(), hi.trim().parse::<i64>()) {
                return json!({ "type": "integer", "minimum": lo, "maximum": hi, "description": token });
            }
        }
    }
    match token {
        "int" | "number" | "u8" | "u64" => json!({ "type": "integer", "description": token }),
        "string" | "text" => json!({ "type": "string", "description": token }),
        "bool" => json!({ "type": "boolean", "description": token }),
        "any" => json!({ "description": "any tree value" }),
        _ if token.starts_with("[[") => json!({
            "type": "array",
            "items": { "type": "array", "items": { "type": "integer" }, "minItems": 2, "maxItems": 2 },
            "description": token,
        }),
        _ => json!({ "type": "string", "description": token }),
    }
}

fn invoke(os: &mut Os, params: &Json, names: &HashMap<String, String>) -> Result<Json, String> {
    let Some(name) = params["name"].as_str() else {
        return Err("tools/call needs a name".to_string());
    };
    let arguments = if params["arguments"].is_object() {
        params["arguments"].clone()
    } else {
        json!({})
    };
    if name == READ_TOOL {
        let path = arguments["path"].as_str().unwrap_or("").to_string();
        let shape = if arguments["shape"].is_object() {
            Some(arguments["shape"].clone())
        } else {
            None
        };
        let text = match os.read(&path, shape.as_ref()) {
            Some(value) => value.to_string(),
            None => "null".to_string(),
        };
        return Ok(json!({
            "content": [{ "type": "text", "text": text }],
            "isError": false,
        }));
    }
    let Some(verb) = names.get(name) else {
        return Err(format!("no such tool: {name}"));
    };
    let mut args = arguments;
    let now = args["now"].as_i64();
    if let Some(fields) = args.as_object_mut() {
        fields.remove("now");
    }
    let mut made = Call::new(verb, args);
    made = made.at(now.unwrap_or_else(crate::now_ms));
    let outcome = os.call(made);
    let envelope = os.envelope(None);
    let told = json!({
        "ok": outcome.ok,
        "data": outcome.data,
        "note": outcome.note,
        "tick": envelope.tick,
        "route": envelope.route.as_ref().map(|r| r.app.clone()),
    });
    Ok(json!({
        "content": [{ "type": "text", "text": told.to_string() }],
        "isError": !outcome.ok,
    }))
}

fn resources(os: &Os) -> Json {
    let surface = os.list(None);
    let mut out = vec![json!({
        "uri": "mrly://read/",
        "name": "envelope",
        "title": "🌍 the envelope",
        "description": "tick, route, focused view, last outcome",
        "mimeType": "application/json",
    })];
    for app in surface["apps"].as_array().cloned().unwrap_or_default() {
        if app["hidden"].as_bool() == Some(true) {
            continue;
        }
        let Some(route) = app["route"].as_str() else {
            continue;
        };
        out.push(json!({
            "uri": format!("mrly://read/{route}"),
            "name": route,
            "title": format!("{} {}", app["emoji"].as_str().unwrap_or(""), app["title"].as_str().unwrap_or(route)),
            "description": app["category"].as_str().unwrap_or(""),
            "mimeType": "application/json",
        }));
    }
    json!({ "resources": out })
}

fn fetch(os: &Os, params: &Json) -> Result<Json, String> {
    let Some(uri) = params["uri"].as_str() else {
        return Err("resources/read needs a uri".to_string());
    };
    let Some(path) = uri.strip_prefix("mrly://read/") else {
        return Err(format!("unknown uri: {uri}"));
    };
    let Some(value) = os.read(path, None) else {
        return Err(format!("nothing at {path}"));
    };
    Ok(json!({
        "contents": [{
            "uri": uri,
            "mimeType": "application/json",
            "text": value.to_string(),
        }],
    }))
}
