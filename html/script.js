var canvas = document.getElementById('canvas');
var gl = canvas.getContext('webgl2');
console.log(gl);
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
    log_num: function(x) { console.log(x); },
    ActiveTexture: gl.activeTexture.bind(gl),
    AttachShader: gl.attachShader.bind(gl),
    Clear: gl.clear.bind(gl),
    ClearColor: gl.clearColor.bind(gl),
    CompileShader: gl.compileShader.bind(gl),
    CreateShader: gl.createShader.bind(gl),
    CreateProgram: gl.createProgram.bind(gl),
    BindBuffer: gl.bindBuffer.bind(gl),
    BindTexture: gl.bindTexture.bind(gl),
    BindVertexArray: function(x) { console.log(arguments); },//gl.bindVertexArray.bind(gl),
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
    GenBuffer: gl.createBuffer.bind(gl),
    GenerateMipmap: gl.generateMipmap.bind(gl),
    GenTexture: gl.createTexture.bind(gl),
    GenVertexArray: gl.createVertexArray.bind(gl),
    GetAttribLocation: gl.getAttribLocation.bind(gl),
    GetError: gl.getError.bind(gl),
    GetShaderInfoLog: gl.getShaderInfoLog.bind(gl),
    GetShaderiv: gl.getShaderParameter.bind(gl),
    GetUniformLocation: gl.getUniformLocation.bind(gl),
    LinkProgram: gl.linkProgram.bind(gl),
    ShaderSource: gl.shaderSource.bind(gl),
    TexParameteri: gl.texParameteri.bind(gl),
    Uniform1i: gl.uniform1i.bind(gl),
    UseProgram: gl.useProgram.bind(gl),
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
