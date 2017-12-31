var canvas = document.getElementById('canvas');
var gl = canvas.getContext('webgl2');
console.log(gl);
var loaded_image = {}
var gl_objects = [];

const DEFAULT_VERTEX_SHADER = `
attribute vec2 position;
varying vec2 tex_coord;
varying vec4 color;
varying lowp float uses_texture;
void main() {
    gl_Position = vec4(position, 0, 1);
}`;

const DEFAULT_FRAGMENT_SHADER = `
varying highp vec4 color;
varying highp vec2 tex_coord;
varying lowp float uses_texture;
uniform sampler2D tex;
void main() {
    highp vec4 tex_color = (int(uses_texture) != 0) ? texture2D(tex, tex_coord) : vec4(1, 1, 1, 1);
    gl_FragColor = color * tex_color;
}`;

var env = {
    create_context: function(width, height) {
        canvas.width = width;
        canvas.height = height;
        gl.viewportWidth = width;
        gl.viewportHeight = height;
    },
    load_image: function(string) {
        var images = document.getElementsByTagName("img");
        var image = null;
        for(var i = 0; i < images.length; i++) {
            if(images[i].src === string)
                image = images[i];
        }
        if(image == null) return false;
        var texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
        gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
        gl.TexImage2D(gl.TEXTURE_2D, 0, gl.RGBA, image.width, image.height, 0, format, gl.UNSIGNED_BYTE, image);
        gl.GenerateMipmap(gl.TEXTURE_2D);
        loaded_image = { 
            id: texture,
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
    load_vertex_shader: (index) => gl.shaderSource(gl_objects[index], DEFAULT_VERTEX_SHADER),
    load_frag_shader: (index) => gl.shaderSource(gl_objects[index], DEFAULT_FRAGMENT_SHADER),
    log_num: function(x) { console.log(x); },
    ActiveTexture: gl.activeTexture.bind(gl),
    AttachShader: (progindex, shadeindex) => gl.attachShader(gl_objects[progindex], gl_objects[shadeindex]),
    Clear: gl.clear.bind(gl),
    ClearColor: gl.clearColor.bind(gl),
    CompileShader: (index) => gl.compileShader(gl_objects[index]),
    CreateShader: (type) => gl_objects.push(gl.createShader(type)) - 1,
    CreateProgram: () => gl_objects.push(gl.createProgram()) - 1,
    BindBuffer: (mask, index) => gl.bindBuffer(mask, gl_objects[index]),
    BindTexture: (index) => gl.bindTexture(gl_objects[index]),
    BindVertexArray: (index) => gl.bindVertexArray(gl_objects[index]),
    BlendFunc: gl.blendFunc.bind(gl),
    BufferData: gl.bufferData.bind(gl), //likely to cause problems
    BufferSubData: gl.bufferSubData.bind(gl), // likely to cause problems
    DeleteBuffer: gl.deleteBuffer.bind(gl),
    DeleteProgram: gl.deleteProgram.bind(gl),
    DeleteShader: gl.deleteShader.bind(gl),
    DeleteTexture: gl.deleteTexture.bind(gl),
    DeleteVertexArray: gl.deleteVertexArray.bind(gl),
    DrawElements: gl.drawElements.bind(gl), // likely to cause problems
    Enable: gl.enable.bind(gl),
    EnableVertexAttribArray: gl.enableVertexAttribArray.bind(gl),
    GenBuffer: () => gl_objects.push(gl.createBuffer()) - 1,
    GenerateMipmap: () => gl_objects.push(gl.generateMipmap()) - 1,
    GenTexture: () => gl_objects.push(gl.createTexture()) - 1,
    GenVertexArray: () => gl_objects.push(gl.createVertexArray()) - 1,
    GetAttribLocation: gl.getAttribLocation.bind(gl),
    GetError: gl.getError.bind(gl),
    GetShaderInfoLog: (index) => { var str = gl.getShaderInfoLog(gl_objects[index]); console.log(str); return str; },
    GetShaderiv: (index, param) => gl.getShaderParameter(gl_objects[index], param),
    GetUniformLocation: gl.getUniformLocation.bind(gl),
    LinkProgram: (index) => gl.linkProgram(gl_objects[index]),
    ShaderSource: (shader, source) => gl.shaderSource(gl_objects[shader], source),
    TexParameteri: gl.texParameteri.bind(gl),
    Uniform1i: gl.uniform1i.bind(gl),
    UseProgram: (index) => gl.useProgram(gl_objects[index]),
    VertexAttribPointer: gl.vertexAttribPointer.bind(gl),
    Viewport: gl.viewport.bind(gl)
}

fetch("wasm.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes =>  WebAssembly.instantiate(bytes, { env } ))
    .then(results => {
        var init = results.instance.exports.init;
        var draw = results.instance.exports.draw;
        init();
        draw();
    })
