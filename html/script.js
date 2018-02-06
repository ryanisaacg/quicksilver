let canvas = document.getElementById('canvas');
let gl = canvas.getContext('webgl2');
let gl_objects = [];
let instance = {};
function rust_ptr_to_buffer(pointer) {
    const memory = instance.exports.memory;
    return new Uint8Array(memory.buffer, pointer);
}
function rust_ptr_to_int32(pointer) {
    const memory = instance.exports.memory;
    return new Int32Array(memory.buffer, pointer);
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
const key_queue = []
const mouse = { x: 0, y: 0, scroll_type: 0, scroll_x: 0, scroll_y: 0 };
const mouse_queue = []
const assets = []
const music = { playing: null, volume: 1 }
//Establish all of the event hooks
document.addEventListener('keydown', (event) => { 
    if(keycodes[event.code] !== undefined) { 
        key_queue.push(keycodes[event.code] + 1);
        event.preventDefault();
    }
})
document.addEventListener('keyup', (event) => {
    if(keycodes[event.code] !== undefined) {
        key_queue.push(-keycodes[event.code] - 1);
        event.preventDefault();
    }
})
canvas.addEventListener('mousemove', (event) => {
    mouse.x = event.clientX;
    mouse.y = event.clientY;
})
canvas.addEventListener('mousedown', (event) => {
    if(event.button < 3) {
        mouse_queue.push(event.button + 1);
        event.preventDefault();
    }
})
canvas.addEventListener('mouseup', (event) => {
    if(event.button < 3) {
        mouse_queue.push(-event.button - 1);
        event.preventDefault();
    }
})
canvas.addEventListener('wheel', (event) => {
    mouse.scroll_type = event.deltaMode;
    mouse.scroll_x = event.deltaX;
    mouse.scroll_y = event.deltaY;
    event.preventDefault();
})
let env = {
    is_texture_loaded: (index) => assets[index].loaded,
    is_texture_errored: (index) => assets[index].error,
    is_sound_loaded: (index) => assets[index].loaded,
    is_sound_errored: (index) => assets[index].error,
    is_text_file_loaded: (index) => assets[index].loaded,
    is_text_file_errored: (index) => assets[index].error,
    fmodf: (a, b) => a % b,
    pump_key_queue: () => key_queue.length > 0 ? key_queue.shift() : 0,
    pump_mouse_queue: () => mouse_queue.length > 0 ? mouse_queue.shift() : 0,
    get_mouse_x: () => mouse.x,
    get_mouse_y: () => mouse.y,
    mouse_scroll_clear: () => { mouse.scroll_x = 0; mouse.scroll_y = 0; },
    mouse_scroll_type: () => mouse.scroll_type,
    mouse_scroll_x: () => mouse.scroll_x,
    mouse_scroll_y: () => mouse.scroll_y,
    print: (pointer) => console.log(rust_str_to_js(pointer)),
    printnum: (x) => console.log(x),
    set_show_mouse: (show) => canvas.style.cursor = show ? "auto" : "none",
    create_context: (title, width, height) => {
        document.title = rust_str_to_js(title);
        canvas.width = width;
        canvas.height = height;
        gl.viewportWidth = width;
        gl.viewportHeight = height;
    },
    set_title: (title) => document.title = rust_str_to_js(title),
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
    load_sound: (pointer) => {
        const sound = new Audio(rust_str_to_js(pointer));
        sound.play()
        console.log(sound)
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
    load_text_file: (ptr) => {
        const index = assets.push({}) - 1;
        fetch(rust_str_to_js(ptr))
            .then(response => response.text())
            .then(string => {
                assets[index].loaded = true;
                assets[index].value = string;
            })
            .catch(err => assets[index].error = true);
        return index;
    },
    text_file_contents: (index) => js_str_to_rust(assets[index].value),
    play_sound: (index, volume) => {
        const sound = assets[index].sound.clone();
        sound.volume = volume;
        sound.play();
    },
    set_music_track: (index) => {
        if(music.playing) { 
            music.playing.stop()
        }
        const source = assets[music.index].sound.clone();
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
    get_image_id: (index) => assets[index].id,
    get_image_width: (index) => assets[index].width,
    get_image_height: (index) => assets[index].height,
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
    log_num: function(x) { console.log(x); },
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
    Viewport: gl.viewport.bind(gl)
}

fetch("wasm.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes =>  WebAssembly.instantiate(bytes, { env } ))
    .then(results => {
        instance = results.instance;
        let init = instance.exports.init;
        let state_ptr = init();
        function update() {
            let delay = instance.exports.update(state_ptr);
            setTimeout(update, delay);
        }
        function draw() {
            instance.exports.draw(state_ptr);
            requestAnimationFrame(draw);
        }
        update();
        requestAnimationFrame(draw);
    })
