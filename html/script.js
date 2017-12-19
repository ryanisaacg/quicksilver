var canvas = document.getElementById('canvas');
var gl = canvas.getContext('webgl');
var loaded_image = {}
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
    ActiveTexture: gl.activeTexture,
    AttachShader: gl.attachShader,
    Clear: gl.clear,
    ClearColor: gl.clearColor,
    CompileShader: gl.compileShader,
    CreateShader: gl.createShader,
    CreateProgram: gl.createProgram,
    BindBuffer: gl.bindBuffer,
    BindTexture: gl.bindTexture,
    BindVertexArray: gl.bindVertexArray,
    BlendFunc: gl.blendFunc,
    BufferData: gl.bufferData, //likely to cause problems
    BufferSubData: gl.bufferSubData, // likely to cause problems
    DeleteBuffer: gl.deleteBuffer,
    DeleteProgram: gl.deleteProgram,
    DeleteShader: gl.deleteShader,
    DeleteTexture: gl.deleteTexture,
    DeleteVertexArray: gl.deleteVertexArray,
    DrawElements: gl.drawElements, // likely to cause problems
    Enable: gl.enable,
    EnableVertexAttribArray: gl.enableVertexAttribArray,
    GenBuffer: gl.genBuffer,
    GenerateMipmap: gl.generateMipmap,
    GenTexture: gl.genTexture,
    GenVertexArray: gl.genVertexArray,
    GetAttribLocation: gl.getAttribLocation,
    GetError: gl.getError,
    GetShaderInfoLog: gl.getShaderInfoLog,
    GetShaderiv: gl.getShaderiv,
    GetUniformLocation: gl.getUniformLocation,
    LinkProgram: gl.linkProgram,
    ShaderSource: gl.shaderSource,
    TexParameteri: gl.texParameteri,
    Uniform1i: gl.uniform1i,
    UseProgram: gl.useProgram,
    VertexAttribPointer: gl.vertexAttribPointer,
    Viewport: gl.viewport
}
fetch("wasm.wasm")
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, { env } ))
    .then(results => {
        var init = results.instance.exports.init;
        var draw = results.instance.exports.draw;
        init();
        draw();
    })
