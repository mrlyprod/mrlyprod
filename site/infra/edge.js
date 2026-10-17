var APEX = 'mrly.net';
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

function handler(event) {
    var request = event.request;
    var host = request.headers.host ? request.headers.host.value : '';
    var tail = search(request.querystring);
    if (host === 'www.' + APEX) {
        return redirect('https://' + APEX + request.uri + tail);
    }
    var step = decide(request.uri);
    if (step.redirect) {
        return redirect(step.redirect + tail);
    }
    request.uri = step.uri;
    return request;
}

function redirect(location) {
    return {
        statusCode: 301,
        statusDescription: 'Moved Permanently',
        headers: { 'location': { value: location } }
    };
}
