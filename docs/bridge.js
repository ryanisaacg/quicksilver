// EDIT THIS LINE TO THE FILENAME OF YOUR WASM BINARY
// YOU CAN ALSO DEFINE THIS VARIABLE BEFORE THE SCRIPT RUNS
WASM_FILE_LOCATION = window.WASM_FILE_LOCATION || 'wasm.wasm'
// PROBABLY DON'T EDIT ANYTHING BELOW THIS LINE
let canvas = document.getElementById('canvas');
let gl = canvas.getContext('webgl2');
let gl_objects = [];
let instance = {};
const init = { window: null, state: null };
function get_data_view() {
    return new DataView(instance.exports.memory.buffer);
}
function rust_ptr_to_buffer(pointer) {
    return new Uint8Array(instance.exports.memory.buffer, pointer);
}
function rust_ptr_to_int32(pointer) {
    return new Int32Array(instance.exports.memory.buffer, pointer);
}
function rust_str_to_js(pointer) {
    const buffer = rust_ptr_to_buffer(pointer);
    let string = '';
    for(let i = 0; buffer[i] != 0; i++)
        string += String.fromCharCode(buffer[i]);
    instance.exports.deallocate_cstring(pointer);
    return string;
}
function js_str_to_rust(string) {
    const pointer = instance.exports.allocate_memory(string.length + 1);
    const buffer = rust_ptr_to_buffer(pointer);
    for(let i = 0; i < string.length; i++) {
        buffer[i] = string.charCodeAt(i)
    }
    buffer[string.length] = 0;
    return pointer;
}
//Generate the keycodes from an in-order list that maps to the Glutin keycodes in rust
const keynames = ["Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Digit6", "Digit7", "Digit8", "Digit9", "Digit0", "KeyA", "KeyB", "KeyC", "KeyD", "KeyE", "KeyF", "KeyG", "KeyH", "KeyI", "KeyJ", "KeyK", "KeyL", "KeyM", 
    "KeyN", "KeyO", "KeyP", "KeyQ", "KeyR", "KeyS", "KeyT", "KeyU", "KeyV", "KeyW", "KeyX", "KeyY", "KeyZ", "Escape", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", 
    "F13", "F14", "F15", "PrintScreen", "ScrollLock", "Pause", "Insert", "Home", "Delete", "End", "PageDown", "PageUp", "ArrowLeft", "ArrowUp", "ArrowRight", 
    "ArrowDown", "Backspace", "Enter", "Space", "Compose", "Caret", "NumLock", "Numpad0", "Numpad1", "Numpad2", "Numpad3", "Numpad4", "Numpad5", 
    "Numpad6", "Numpad7", "Numpad8", "Numpad9", "AbntC1", "AbntC2", "Add", "Quote", "Apps", "At", "Ax", "Backslash", "Calculator", 
    "Capital", "Colon", "Comma", "Convert", "Decimal", "Divide", "Equal", "Backquote", "Kana", "Kanji", "AltLeft", "BracketLeft", "ControlLeft", 
    "LMenu", "ShiftLeft", "MetaLeft", "Mail", "MediaSelect", "MediaStop", "Minus", "Multiply", "Mute", "LaunchMyComputer", "NavigateForward", 
    "NavigateBackward", "NextTrack", "NoConvert", "NumpadComma", "NumpadEnter", "NumpadEquals", "OEM102", "Period", "PlayPause", 
    "Power", "PrevTrack", "AltRight", "BracketRight", "ControlRight", "RMenu", "ShiftRight", "MetaRight", "Semicolon", "Slash", "Sleep", "Stop", "Subtract", 
    "Sysrq", "Tab", "Underline", "Unlabeled", "AudioVolumeDown", "AudioVolumeUp", "Wake", "WebBack", "WebFavorites", "WebForward", "WebHome", 
    "WebRefresh", "WebSearch", "WebStop", "Yen"];
const keycodes = keynames.reduce((map, value, index) => { map[value] = index; return map }, {})
const assets = []
const music = { playing: null, volume: 1 }
const event_data = { button: 0, state: 0, f1: 0, f2: 0, id: 0 };
let send_event = () => {};
//Establish all of the event hooks
window.onclose = () => send_event(0);
window.onfocus = () => send_event(1);
window.onblur = () => send_event(2);
document.onkeydown = (event) => {
    if(keycodes[event.code] !== undefined) {
        event_data.button = keycodes[event.code];
        event_data.state = 0;
        send_event(3);
        event.preventDefault();
    }
};
document.onkeyup = (event) => {
    if(keycodes[event.code] !== undefined) {
        event_data.button = keycodes[event.code];
        event_data.state = 2;
        send_event(3);
        event.preventDefault();
    }
};
canvas.onmousemove = (event) => {
    event_data.f1 = event.offsetX;
    event_data.f2 = event.offsetY;
    send_event(4);
};
canvas.onmouseenter = () => send_event(5);
canvas.onmouseleave = () => send_event(6);
canvas.onwheel = (event) => {
    event_data.f1 = event.deltaX;
    event_data.f2 = event.deltaY;
    send_event(7);
    event.preventDefault();
}
document.onmousedown = (event) => {
    if(event.button < 3) {
        event_data.button = event.button;
        event_data.state = 0;
        send_event(8);
    }
}
canvas.onmouseup = (event) => {
    if(event.button < 3) {
        event_data.button = event.button;
        event_data.state = 2;
        send_event(8);
    }
}
let env = {
    // Windowing
    set_show_mouse: (show) => canvas.style.cursor = show ? "auto" : "none",
    get_page_width: () => document.body.clientWidth,
    get_page_height: () => document.body.clientHeight,
    create_context: (title, width, height) => {
        document.title = rust_str_to_js(title);
        canvas.width = width;
        canvas.height = height;
        gl.viewportWidth = width;
        gl.viewportHeight = height;
    },
    set_title: (title) => document.title = rust_str_to_js(title),
    //Event data
    event_data_button: () => event_data.button,
    event_data_state: () => event_data.state,
    event_data_f1: () => event_data.f1,
    event_data_f2: () => event_data.f2,
    event_data_id: () => event_data.id,
    //Gamepads
    gamepad_count: () => navigator.getGamepads().length,
    gamepad_data: (start, id, buttons, axes, next) => {
        const gamepads = navigator.getGamepads();
        const data = get_data_view();
        for(let i = 0; i < gamepads.length; i++) {
            const offset = (next - start) * i;
            const gamepad = gamepads[i];
            data.setUint32(id + offset, gamepad.index);
            //TODO: prevent garbage data from happening
            for(let j = 0; j < gamepad.buttons.length && j < 17; j++) {
                const value = gamepad.buttons[j].pressed ? 1 : 3;
                data.setUint8(buttons + j + offset, value);
            }
            for(let j = 0; j < gamepad.axes.length && j < 4; j++) {
                data.setFloat32(axes + 3 + j * 4 + offset, gamepad.axes[j]);
            }
        }
    },
    // Saving / loading
    save_cookie: (key_ptr, val_ptr) => document.cookie = rust_str_to_js(key_ptr) + "=" + rust_str_to_js(val_ptr) + ";",
    load_cookie: (key_ptr) => {
        const key = rust_str_to_js(key_ptr);
        const name = key + "=";
        const decodedCookie = decodeURIComponent(document.cookie);
        const ca = decodedCookie.split(';');
        let value = '';
        for(let i = 0; i < ca.length; i++) {
            let c = ca[i];
            while (c.charAt(0) == ' ') {
                c = c.substring(1);
            }
            if (c.indexOf(name) == 0) {
                value = c.substring(name.length, c.length);
            }
        }
        return js_str_to_rust(value);
    },
    //Sounds
    load_sound: (pointer) => {
        const sound = new Audio(rust_str_to_js(pointer));
        const index = assets.push({ loaded: false }) - 1;
        sound.oncanplaythrough = () => {
            assets[index].loaded = true;
            assets[index].sound = sound;
        }
        if(sound.readyState === 4) {
            sound.oncanplaythrough()
        }
        sound.onerror = () => {
            assets[index].loaded = true;
            assets[index].error = true;
        }
        return index;
    },
    play_sound: (index, volume) => {
        const sound = assets[index].sound.cloneNode();
        sound.volume = volume;
        sound.play();
    },
    set_music_track: (index) => {
        if(music.playing) { 
            music.playing.stop()
        }
        const source = assets[music.index].sound.cloneNode();
        source.loop = true;
        source.volume = music.volume;
        source.play();
        music.playing = source;
    },
    play_music: () => { 
        if(music.playing) { 
            music.playing.play();
        }
    },
    pause_music: () => {
        if(music.playing) {
            music.playing.pause();
        }
    },
    get_music_volume: () => music.volume,
    set_music_volume: (volume) => {
        music.volume = volume;
        if(music.playing) {
            music.playing.volume = volume;
        }
    },
    // Images
    load_image: (pointer) => {
        const image = new Image();
        image.src = rust_str_to_js(pointer);
        const index = assets.push({ loaded: false }) - 1;
        image.onload = () => {
            let texture = gl.createTexture();
            gl.bindTexture(gl.TEXTURE_2D, texture);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
            gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, image.width, image.height, 0, gl.RGBA, gl.UNSIGNED_BYTE, image);
            gl.generateMipmap(gl.TEXTURE_2D);
            const id = gl_objects.push(texture) - 1;
            assets[index].loaded = true;
            assets[index].error = false;
            assets[index].id = id;
            assets[index].width = image.width;
            assets[index].height = image.height;
        }
        if(image.complete) { 
            image.onload()
        }
        image.onerror = () => {
            assets[index].loaded = true;
            assets[index].error = true;
        }
        return index;
    },
    get_image_id: (index) => assets[index].id,
    get_image_width: (index) => assets[index].width,
    get_image_height: (index) => assets[index].height,
    //Arbitrary files
    load_file: (ptr) => {
        const index = assets.push({}) - 1;
        fetch(rust_str_to_js(ptr))
            .then(response => response.arrayBuffer())
            .then(bytes => {
                const pointer = instance.exports.allocate_memory(bytes.byteLength);
                const target = new Uint8Array(instance.exports.memory.buffer, pointer, bytes.byteLength);
                const source = new Uint8Array(bytes);
                for(let i = 0; i < source.length; i++) {
                    target[i] = source[i];
                }
                assets[index].loaded = true;
                assets[index].contents = pointer;
                assets[index].length = source.length;
            })
            .catch(err => { console.log(err); assets[index].error = true });
        return index;
    },
    file_contents: (index) => assets[index].contents,
    file_length: (index) => assets[index].length,
    //Asset loading
    ffi_asset_status: (index) => assets[index].error ? 2 : (assets[index].loaded ? 1 : 0),
    //Game loop
    set_app: (app) => {
        setInterval(() => instance.exports.update(app), 16);
        send_event = (event) => {
            instance.exports.event(app, event);
        };
        function draw() {
            instance.exports.draw(app);
            requestAnimationFrame(draw);
        }
        requestAnimationFrame(draw);
    },
    //Rust runtime
    fmodf: (a, b) => a % b,
    sinf: (x) => Math.sin(x),
    cosf: (x) => Math.cos(x),
    tanf: (x) => Math.tan(x),
    pow: (x, y) => Math.pow(x, y),
    roundf: (x) => Math.round(x),
    Math_acos: (x) => Math.acos(x),
    Math_asin: (x) => Math.asin(x),
    Math_atan: (x) => Math.atan(x),
    Math_atan2: (x, y) => Math.atan2(x, y),
    // OpenGL
    ActiveTexture: gl.activeTexture.bind(gl),
    AttachShader: (progindex, shadeindex) => gl.attachShader(gl_objects[progindex], gl_objects[shadeindex]),
    Clear: gl.clear.bind(gl),
    ClearColor: gl.clearColor.bind(gl),
    CompileShader: (index) => gl.compileShader(gl_objects[index]),
    CreateShader: (type) => gl_objects.push(gl.createShader(type)) - 1,
    CreateProgram: () => gl_objects.push(gl.createProgram()) - 1,
    BindBuffer: (mask, index) => gl.bindBuffer(mask, gl_objects[index]),
    BindFramebuffer: (target, index) => gl.bindFramebuffer(target, index == 0 ? null : gl_objects[index]),
    BindTexture: (target, index) => gl.bindTexture(target, gl_objects[index]),
    BindVertexArray: (index) => gl.bindVertexArray(gl_objects[index]),
    BlendEquationSeparate: gl.blendEquationSeparate.bind(gl),
    BlendFunc: gl.blendFunc.bind(gl),
    BufferData: (target, size, data, usage) => gl.bufferData(target, rust_ptr_to_buffer(data), usage, 0, size),
    BufferSubData: (target, offset, size, data) => gl.bufferSubData(target, offset, rust_ptr_to_buffer(data), 0, size), 
    DeleteBuffer: (index) => gl.deleteBuffer(gl_objects[index]),
    DeleteFramebuffer: (index) => gl.deleteFramebuffer(gl_objects[index]),
    DeleteProgram: (index) => gl.deleteProgram(gl_objects[index]),
    DeleteShader: (index) => gl.deleteShader(gl_objects[index]),
    DeleteTexture: (index) => gl.deleteTexture(gl_objects[index]),
    DeleteVertexArray: (index) => gl.deleteVertexArray(gl_objects[index]),
    DrawBuffer: (buffer) => gl.drawBuffers([buffer]),
    DrawElements: gl.drawElements.bind(gl), 
    Enable: gl.enable.bind(gl),
    EnableVertexAttribArray: gl.enableVertexAttribArray.bind(gl),
    FramebufferTexture: (target, attachment, tex_index, level) => gl.framebufferTexture2D(target, attachment, gl.TEXTURE_2D, gl_objects[tex_index], level),
    GenBuffer: () => gl_objects.push(gl.createBuffer()) - 1,
    GenerateMipmap: gl.generateMipmap.bind(gl),
    GenFramebuffer: () => gl_objects.push(gl.createFramebuffer()) - 1,
    GenTexture: () => gl_objects.push(gl.createTexture()) - 1,
    GenVertexArray: () => gl_objects.push(gl.createVertexArray()) - 1,
    GetAttribLocation: (index, string_ptr) => gl.getAttribLocation(gl_objects[index], rust_str_to_js(string_ptr)),    
    GetShaderInfoLog: (index) => { let str = gl.getShaderInfoLog(gl_objects[index]); console.log(str); return str; }, //todo: convert to rust string?
    GetShaderiv: (index, param) => gl.getShaderParameter(gl_objects[index], param),
    GetUniformLocation: (index, string_ptr) => gl_objects.push(gl.getUniformLocation(gl_objects[index], rust_str_to_js(string_ptr))) - 1,
    GetViewport: (ptr) => {
        const buffer = rust_ptr_to_int32(ptr);
        const values = gl.getParameter(gl.VIEWPORT);
        for(let i = 0; i < 4; i++) {
            buffer[i] = values[i];
        }
    },
    LinkProgram: (index) => gl.linkProgram(gl_objects[index]),
    ShaderSource: (shader, source_ptr) => gl.shaderSource(gl_objects[shader], rust_str_to_js(source_ptr)),
    TexImage2D: (target, level, internal, width, height, border, format, textype, data) => 
        gl_objects.push(gl.texImage2D(target, level, internal, width, height, border, format, textype, 
                                      data == 0 ? new Uint8Array(width * height * 4) : rust_ptr_to_buffer(data), 0)) - 1,
    TexParameteri: (target, pname, param) => gl.texParameteri.bind(gl),
    Uniform1i: (index, value) => gl.uniform1i(gl_objects[index], value),
    UseProgram: (index) => gl.useProgram(gl_objects[index]),
    VertexAttribPointer: gl.vertexAttribPointer.bind(gl),
    Viewport: gl.viewport.bind(gl),
}

fetch(WASM_FILE_LOCATION)
    .then(response => response.arrayBuffer())
    .then(bytes =>  WebAssembly.instantiate(bytes, { env } ))
    .then(results => {
        instance = results.instance;
        instance.exports.main();
    })
