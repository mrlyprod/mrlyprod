/*LOCK*/
var MOVED = /*MOVED*/{};

function decide(uri) {
    var to = MOVED[uri] || MOVED[uri + '/'];
    if (to) {
        return { redirect: to };
    }
    if (uri.indexOf('/raw/') === 0) {
        return { uri: uri };
    }
    if (uri.indexOf('/git/') === 0) {
        return { uri: '/index.html' };
    }
    var last = uri.substring(uri.lastIndexOf('/') + 1);
    if (last.indexOf('.') !== -1) {
        return { uri: uri };
    }
    if (last === '') {
        return { uri: '/index.html' };
    }
    return { redirect: uri + '/' };
}

function join(to, tail) {
    if (!tail || to.indexOf('?') === -1) {
        return to + tail;
    }
    return to + '&' + tail.substring(1);
}

function search(query) {
    var parts = [];
    for (var key in query) {
        var list = query[key].multiValue || [query[key]];
        for (var i = 0; i < list.length; i++) {
            parts.push(key + '=' + list[i].value);
        }
    }
    return parts.length ? '?' + parts.join('&') : '';
}

async function handler(event) {
    var request = event.request;
    var host = request.headers.host ? request.headers.host.value : '';
    var tail = search(request.querystring);
    if (host.indexOf('www.') === 0) {
        return redirect('https://' + host.substring(4) + request.uri + tail);
    }
    if (typeof gate === 'function') {
        var shut = await guard(request);
        if (shut.status) {
            return respond(shut);
        }
        if (!shut.pass) {
            request.uri = shut.uri;
            return request;
        }
    }
    var step = decide(request.uri);
    if (step.redirect) {
        return redirect(join(step.redirect, tail));
    }
    request.uri = step.uri;
    return request;
}

async function guard(request) {
    var kit = { hmac: hmac, sha256: sha256 };
    var names = needs(request.uri);
    for (var i = 0; i < names.length; i++) {
        try {
            kit[names[i]] = await kvs.get(names[i]);
        } catch (e) {
            kit[names[i]] = '';
        }
    }
    var cookie = request.cookies && request.cookies[COOKIE] ? request.cookies[COOKIE].value : '';
    var said = request.headers[HEADER] ? request.headers[HEADER].value : '';
    var ask = { uri: request.uri, cookie: cookie, password: said, now: Math.floor(Date.now() / 1000) };
    return gate(ask, kit);
}

function hmac(key, text) {
    return crypto.createHmac('sha256', key).update(text).digest('hex');
}

function sha256(text) {
    return crypto.createHash('sha256').update(text).digest('hex');
}

function respond(shut) {
    var out = { statusCode: shut.status, headers: { 'cache-control': { value: 'no-store' } }, cookies: {} };
    if (shut.location) {
        out.headers.location = { value: shut.location };
    }
    if (shut.cookie) {
        out.cookies[COOKIE] = shut.cookie;
    }
    return out;
}

function redirect(location) {
    return {
        statusCode: 301,
        statusDescription: 'Moved Permanently',
        headers: { 'location': { value: location } }
    };
}
