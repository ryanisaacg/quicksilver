let canvas = document.getElementById('canvas');
let gl = canvas.getContext('webgl2');
console.log(gl);
let loaded_image = {}
let gl_objects = [];

const DEFAULT_VERTEX_SHADER = `
attribute vec2 position;
letying vec2 tex_coord;
letying vec4 color;
letying lowp float uses_texture;
void main() {
    gl_Position = vec4(position, 0, 1);
}`;

const DEFAULT_FRAGMENT_SHADER = `
letying highp vec4 color;
letying highp vec2 tex_coord;
letying lowp float uses_texture;
uniform sampler2D tex;
void main() {
    highp vec4 tex_color = (int(uses_texture) != 0) ? texture2D(tex, tex_coord) : vec4(1, 1, 1, 1);
    gl_FragColor = color * tex_color;
}`;
let instance = {};
function rust_ptr_to_buffer(pointer) {
    const memory = instance.exports.memory;
    return new Uint8Array(memory.buffer, pointer);
}
function rust_str_to_js(pointer) {
    const buffer = rust_ptr_to_buffer(pointer);
    let string = '';
    for(let i = 0; buffer[i] != 0; i++)
        string += String.fromCharCode(buffer[i]);
    return string;
}
//Generate the keycodes from an in-order list that maps to the Glutin keycodes in rust
const keynames = ["Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Digit6", "Digit7", "Digit8", "Digit9", "Digit0", "KeyA", "KeyB", "KeyC", "KeyD", "KeyE", "KeyF", "KeyG", "KeyH", "KeyI", "KeyJ", "KeyK", "KeyL", "KeyM", 
    "KeyN", "KeyO", "KeyP", "KeyQ", "KeyR", "KeyS", "KeyT", "KeyU", "KeyV", "KeyW", "KeyX", "KeyY", "KeyZ", "Escape", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", 
    "F13", "F14", "F15", "PrintScreen", "ScrollLock", "Pause", "Insert", "Home", "Delete", "End", "PageDown", "PageUp", "ArrowLeft", "ArrowUp", "ArrowRight", 
    "ArrowDown", "Backspace", "Enter", "Space", "Compose", "NumLock", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", 
    "Numpad6", "Numpad7", "Numpad8", "Numpad9", "AbntC1", "AbntC2", "Add", "Quote", "Apps", "At", "Ax", "Backslash", "Calculator", 
    "Capital", "Colon", "Comma", "Convert", "Decimal", "Divide", "Equal", "Backquote", "Kana", "Kanji", "AltLeft", "BracketLeft", "ControlLeft", 
    "LMenu", "ShiftLeft", "MetaLeft", "Mail", "MediaSelect", "MediaStop", "Minus", "Multiply", "Mute", "LaunchMyComputer", "NavigateForward", 
    "NavigateBackward", "NextTrack", "NoConvert", "NumpadComma", "NumpadEnter", "NumpadEquals", "OEM102", "Period", "PlayPause", 
    "Power", "PrevTrack", "AltRight", "BracketRight", "ControlRight", "RMenu", "ShiftRight", "MetaRight", "Semicolon", "Slash", "Sleep", "Stop", "Subtract", 
    "Sysrq", "Tab", "Underline", "Unlabeled", "AudioVolumeDown", "AudioVolumeUp", "Wake", "WebBack", "WebFavorites", "WebForward", "WebHome", 
    "WebRefresh", "WebSearch", "WebStop", "Yen"];
const keycodes = keynames.reduce((map, value, index) => { map[value] = index; return map }, {})

let env = {
    print: (pointer) => console.log(rust_str_to_js(pointer)),
    create_context: function(width, height) {
        canvas.width = width;
        canvas.height = height;
        gl.viewportWidth = width;
        gl.viewportHeight = height;
    },
    load_image: (pointer) => {
        let string = rust_str_to_js(pointer);
        let image = document.getElementById(string);
        if(image == null) return false;
        let texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, image.width, image.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, image);
        gl.generateMipmap(gl.TEXTURE_2D);
        loaded_image = { 
            id: gl_objects.push(texture) - 1,
            width: image.width,
            height: image.height
        }
        return true;
    },
    get_image_id: function() {
        return loaded_image.id
    },
    get_image_width: function() {
        return loaded_image.width;
    },
    get_image_height: function() {
        return loaded_image.height;
    },
    log_num: function(x) { console.log(x); },
    ActiveTexture: gl.activeTexture.bind(gl),
    AttachShader: (progindex, shadeindex) => gl.attachShader(gl_objects[progindex], gl_objects[shadeindex]),
    Clear: gl.clear.bind(gl),
    ClearColor: gl.clearColor.bind(gl),
    CompileShader: (index) => gl.compileShader(gl_objects[index]),
    CreateShader: (type) => gl_objects.push(gl.createShader(type)) - 1,
    CreateProgram: () => gl_objects.push(gl.createProgram()) - 1,
    BindBuffer: (mask, index) => gl.bindBuffer(mask, gl_objects[index]),
    BindTexture: (target, index) => gl.bindTexture(target, gl_objects[index]),
    BindVertexArray: (index) => gl.bindVertexArray(gl_objects[index]),
    BlendFunc: gl.blendFunc.bind(gl),
    BufferData: (target, size, data, usage) => gl.bufferData(target, rust_ptr_to_buffer(data), usage, 0, size),
    BufferSubData: (target, offset, size, data) => gl.bufferSubData(target, offset, rust_ptr_to_buffer(data), 0, size), 
    DeleteBuffer: (index) => gl.deleteBuffer(gl_objects[index]),
    DeleteProgram: (index) => gl.deleteProgram(gl_objects[index]),
    DeleteShader: (index) => gl.deleteShader(gl_objects[index]),
    DeleteTexture: (index) => gl.deleteTexture(gl_objects[index]),
    DeleteVertexArray: gl.deleteVertexArray.bind(gl),
    DrawElements: gl.drawElements.bind(gl), 
    Enable: gl.enable.bind(gl),
    EnableVertexAttribArray: gl.enableVertexAttribArray.bind(gl),
    GenBuffer: () => gl_objects.push(gl.createBuffer()) - 1,
    GenVertexArray: () => gl_objects.push(gl.createVertexArray()) - 1,
    GetAttribLocation: (index, string_ptr) => gl.getAttribLocation(gl_objects[index], rust_str_to_js(string_ptr)),
    GetShaderInfoLog: (index) => { let str = gl.getShaderInfoLog(gl_objects[index]); console.log(str); return str; }, //todo: convert to rust string?
    GetShaderiv: (index, param) => gl.getShaderParameter(gl_objects[index], param),
    GetUniformLocation: (index, string_ptr) => gl_objects.push(gl.getUniformLocation(gl_objects[index], rust_str_to_js(string_ptr))) - 1,
    LinkProgram: (index) => gl.linkProgram(gl_objects[index]),
    ShaderSource: (shader, source_ptr) => gl.shaderSource(gl_objects[shader], rust_str_to_js(source_ptr)),
    Uniform1i: (index, value) => gl.uniform1i(gl_objects[index], value),
    UseProgram: (index) => gl.useProgram(gl_objects[index]),
    VertexAttribPointer: gl.vertexAttribPointer.bind(gl),
    Viewport: gl.viewport.bind(gl)
}

fetch("wasm.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes =>  WebAssembly.instantiate(bytes, { env } ))
    .then(results => {
        instance = results.instance;
        let init = instance.exports.init;
        let draw = instance.exports.draw;
        init();
        draw();
    })
