"use strict";

if( typeof Rust === "undefined" ) {
    var Rust = {};
}

(function( root, factory ) {
    if( typeof define === "function" && define.amd ) {
        define( [], factory );
    } else if( typeof module === "object" && module.exports ) {
        module.exports = factory();
    } else {
        Rust.lyon = factory();
    }
}( this, function() {
    return (function( module_factory ) {
        var instance = module_factory();

        if( typeof process === "object" && typeof process.versions === "object" && typeof process.versions.node === "string" ) {
            var fs = require( "fs" );
            var path = require( "path" );
            var wasm_path = path.join( __dirname, "lyon.wasm" );
            var buffer = fs.readFileSync( wasm_path );
            var mod = new WebAssembly.Module( buffer );
            var wasm_instance = new WebAssembly.Instance( mod, instance.imports );
            return instance.initialize( wasm_instance );
        } else {
            var file = fetch( "lyon.wasm", {credentials: "same-origin"} );

            var wasm_instance = ( typeof WebAssembly.instantiateStreaming === "function"
                ? WebAssembly.instantiateStreaming( file, instance.imports )
                    .then( function( result ) { return result.instance; } )

                : file
                    .then( function( response ) { return response.arrayBuffer(); } )
                    .then( function( bytes ) { return WebAssembly.compile( bytes ); } )
                    .then( function( mod ) { return WebAssembly.instantiate( mod, instance.imports ) } ) );

            return wasm_instance
                .then( function( wasm_instance ) {
                    var exports = instance.initialize( wasm_instance );
                    console.log( "Finished loading Rust wasm module 'lyon'" );
                    return exports;
                })
                .catch( function( error ) {
                    console.log( "Error loading Rust wasm module 'lyon':", error );
                    throw error;
                });
        }
    }( function() {
    var Module = {};

    Module.STDWEB_PRIVATE = {};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.to_utf8 = function to_utf8( str, addr ) {
    var HEAPU8 = Module.HEAPU8;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        // For UTF8 byte structure, see http://en.wikipedia.org/wiki/UTF-8#Description and https://www.ietf.org/rfc/rfc2279.txt and https://tools.ietf.org/html/rfc3629
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            HEAPU8[ addr++ ] = u;
        } else if( u <= 0x7FF ) {
            HEAPU8[ addr++ ] = 0xC0 | (u >> 6);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0xFFFF ) {
            HEAPU8[ addr++ ] = 0xE0 | (u >> 12);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x1FFFFF ) {
            HEAPU8[ addr++ ] = 0xF0 | (u >> 18);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else if( u <= 0x3FFFFFF ) {
            HEAPU8[ addr++ ] = 0xF8 | (u >> 24);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        } else {
            HEAPU8[ addr++ ] = 0xFC | (u >> 30);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 24) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);
            HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);
            HEAPU8[ addr++ ] = 0x80 | (u & 63);
        }
    }
};

Module.STDWEB_PRIVATE.noop = function() {};
Module.STDWEB_PRIVATE.to_js = function to_js( address ) {
    var kind = Module.HEAPU8[ address + 12 ];
    if( kind === 0 ) {
        return undefined;
    } else if( kind === 1 ) {
        return null;
    } else if( kind === 2 ) {
        return Module.HEAP32[ address / 4 ];
    } else if( kind === 3 ) {
        return Module.HEAPF64[ address / 8 ];
    } else if( kind === 4 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        return Module.STDWEB_PRIVATE.to_js_string( pointer, length );
    } else if( kind === 5 ) {
        return false;
    } else if( kind === 6 ) {
        return true;
    } else if( kind === 7 ) {
        var pointer = Module.STDWEB_PRIVATE.arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var output = [];
        for( var i = 0; i < length; ++i ) {
            output.push( Module.STDWEB_PRIVATE.to_js( pointer + i * 16 ) );
        }
        return output;
    } else if( kind === 8 ) {
        var arena = Module.STDWEB_PRIVATE.arena;
        var value_array_pointer = arena + Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var key_array_pointer = arena + Module.HEAPU32[ (address + 8) / 4 ];
        var output = {};
        for( var i = 0; i < length; ++i ) {
            var key_pointer = Module.HEAPU32[ (key_array_pointer + i * 8) / 4 ];
            var key_length = Module.HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];
            var key = Module.STDWEB_PRIVATE.to_js_string( key_pointer, key_length );
            var value = Module.STDWEB_PRIVATE.to_js( value_array_pointer + i * 16 );
            output[ key ] = value;
        }
        return output;
    } else if( kind === 9 ) {
        return Module.STDWEB_PRIVATE.acquire_js_reference( Module.HEAP32[ address / 4 ] );
    } else if( kind === 10 || kind === 12 || kind === 13 ) {
        var adapter_pointer = Module.HEAPU32[ address / 4 ];
        var pointer = Module.HEAPU32[ (address + 4) / 4 ];
        var deallocator_pointer = Module.HEAPU32[ (address + 8) / 4 ];
        var num_ongoing_calls = 0;
        var drop_queued = false;
        var output = function() {
            if( pointer === 0 || drop_queued === true ) {
                if (kind === 10) {
                    throw new ReferenceError( "Already dropped Rust function called!" );
                } else if (kind === 12) {
                    throw new ReferenceError( "Already dropped FnMut function called!" );
                } else {
                    throw new ReferenceError( "Already called or dropped FnOnce function called!" );
                }
            }

            var function_pointer = pointer;
            if (kind === 13) {
                output.drop = Module.STDWEB_PRIVATE.noop;
                pointer = 0;
            }

            if (num_ongoing_calls !== 0) {
                if (kind === 12 || kind === 13) {
                    throw new ReferenceError( "FnMut function called multiple times concurrently!" );
                }
            }

            var args = Module.STDWEB_PRIVATE.alloc( 16 );
            Module.STDWEB_PRIVATE.serialize_array( args, arguments );

            try {
                num_ongoing_calls += 1;
                Module.STDWEB_PRIVATE.dyncall( "vii", adapter_pointer, [function_pointer, args] );
                var result = Module.STDWEB_PRIVATE.tmp;
                Module.STDWEB_PRIVATE.tmp = null;
            } finally {
                num_ongoing_calls -= 1;
            }

            if( drop_queued === true && num_ongoing_calls === 0 ) {
                output.drop();
            }

            return result;
        };

        output.drop = function() {
            if (num_ongoing_calls !== 0) {
                drop_queued = true;
                return;
            }

            output.drop = Module.STDWEB_PRIVATE.noop;
            var function_pointer = pointer;
            pointer = 0;

            if (function_pointer != 0) {
                Module.STDWEB_PRIVATE.dyncall( "vi", deallocator_pointer, [function_pointer] );
            }
        };

        return output;
    } else if( kind === 14 ) {
        var pointer = Module.HEAPU32[ address / 4 ];
        var length = Module.HEAPU32[ (address + 4) / 4 ];
        var array_kind = Module.HEAPU32[ (address + 8) / 4 ];
        var pointer_end = pointer + length;

        switch( array_kind ) {
            case 0:
                return Module.HEAPU8.subarray( pointer, pointer_end );
            case 1:
                return Module.HEAP8.subarray( pointer, pointer_end );
            case 2:
                return Module.HEAPU16.subarray( pointer, pointer_end );
            case 3:
                return Module.HEAP16.subarray( pointer, pointer_end );
            case 4:
                return Module.HEAPU32.subarray( pointer, pointer_end );
            case 5:
                return Module.HEAP32.subarray( pointer, pointer_end );
            case 6:
                return Module.HEAPF32.subarray( pointer, pointer_end );
            case 7:
                return Module.HEAPF64.subarray( pointer, pointer_end );
        }
    } else if( kind === 15 ) {
        return Module.STDWEB_PRIVATE.get_raw_value( Module.HEAPU32[ address / 4 ] );
    }
};

Module.STDWEB_PRIVATE.serialize_object = function serialize_object( address, value ) {
    var keys = Object.keys( value );
    var length = keys.length;
    var key_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 8 );
    var value_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 8;
    Module.HEAPU32[ address / 4 ] = value_array_pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    Module.HEAPU32[ (address + 8) / 4 ] = key_array_pointer;
    for( var i = 0; i < length; ++i ) {
        var key = keys[ i ];
        var key_address = key_array_pointer + i * 8;
        Module.STDWEB_PRIVATE.to_utf8_string( key_address, key );

        Module.STDWEB_PRIVATE.from_js( value_array_pointer + i * 16, value[ key ] );
    }
};

Module.STDWEB_PRIVATE.serialize_array = function serialize_array( address, value ) {
    var length = value.length;
    var pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );
    Module.HEAPU8[ address + 12 ] = 7;
    Module.HEAPU32[ address / 4 ] = pointer;
    Module.HEAPU32[ (address + 4) / 4 ] = length;
    for( var i = 0; i < length; ++i ) {
        Module.STDWEB_PRIVATE.from_js( pointer + i * 16, value[ i ] );
    }
};

// New browsers and recent Node
var cachedEncoder = ( typeof TextEncoder === "function"
    ? new TextEncoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextEncoder === "function"
        ? new util.TextEncoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedEncoder != null ) {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var buffer = cachedEncoder.encode( value );
        var length = buffer.length;
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.HEAPU8.set( buffer, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };

} else {
    Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {
        var length = Module.STDWEB_PRIVATE.utf8_len( value );
        var pointer = 0;

        if ( length > 0 ) {
            pointer = Module.STDWEB_PRIVATE.alloc( length );
            Module.STDWEB_PRIVATE.to_utf8( value, pointer );
        }

        Module.HEAPU32[ address / 4 ] = pointer;
        Module.HEAPU32[ (address + 4) / 4 ] = length;
    };
}

Module.STDWEB_PRIVATE.from_js = function from_js( address, value ) {
    var kind = Object.prototype.toString.call( value );
    if( kind === "[object String]" ) {
        Module.HEAPU8[ address + 12 ] = 4;
        Module.STDWEB_PRIVATE.to_utf8_string( address, value );
    } else if( kind === "[object Number]" ) {
        if( value === (value|0) ) {
            Module.HEAPU8[ address + 12 ] = 2;
            Module.HEAP32[ address / 4 ] = value;
        } else {
            Module.HEAPU8[ address + 12 ] = 3;
            Module.HEAPF64[ address / 8 ] = value;
        }
    } else if( value === null ) {
        Module.HEAPU8[ address + 12 ] = 1;
    } else if( value === undefined ) {
        Module.HEAPU8[ address + 12 ] = 0;
    } else if( value === false ) {
        Module.HEAPU8[ address + 12 ] = 5;
    } else if( value === true ) {
        Module.HEAPU8[ address + 12 ] = 6;
    } else if( kind === "[object Symbol]" ) {
        var id = Module.STDWEB_PRIVATE.register_raw_value( value );
        Module.HEAPU8[ address + 12 ] = 15;
        Module.HEAP32[ address / 4 ] = id;
    } else {
        var refid = Module.STDWEB_PRIVATE.acquire_rust_reference( value );
        Module.HEAPU8[ address + 12 ] = 9;
        Module.HEAP32[ address / 4 ] = refid;
    }
};

// New browsers and recent Node
var cachedDecoder = ( typeof TextDecoder === "function"
    ? new TextDecoder( "utf-8" )
    // Old Node (before v11)
    : ( typeof util === "object" && util && typeof util.TextDecoder === "function"
        ? new util.TextDecoder( "utf-8" )
        // Old browsers
        : null ) );

if ( cachedDecoder != null ) {
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        return cachedDecoder.decode( Module.HEAPU8.subarray( index, index + length ) );
    };

} else {
    // This is ported from Rust's stdlib; it's faster than
    // the string conversion from Emscripten.
    Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {
        var HEAPU8 = Module.HEAPU8;
        index = index|0;
        length = length|0;
        var end = (index|0) + (length|0);
        var output = "";
        while( index < end ) {
            var x = HEAPU8[ index++ ];
            if( x < 128 ) {
                output += String.fromCharCode( x );
                continue;
            }
            var init = (x & (0x7F >> 2));
            var y = 0;
            if( index < end ) {
                y = HEAPU8[ index++ ];
            }
            var ch = (init << 6) | (y & 63);
            if( x >= 0xE0 ) {
                var z = 0;
                if( index < end ) {
                    z = HEAPU8[ index++ ];
                }
                var y_z = ((y & 63) << 6) | (z & 63);
                ch = init << 12 | y_z;
                if( x >= 0xF0 ) {
                    var w = 0;
                    if( index < end ) {
                        w = HEAPU8[ index++ ];
                    }
                    ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));

                    output += String.fromCharCode( 0xD7C0 + (ch >> 10) );
                    ch = 0xDC00 + (ch & 0x3FF);
                }
            }
            output += String.fromCharCode( ch );
            continue;
        }
        return output;
    };
}

Module.STDWEB_PRIVATE.id_to_ref_map = {};
Module.STDWEB_PRIVATE.id_to_refcount_map = {};
Module.STDWEB_PRIVATE.ref_to_id_map = new WeakMap();
// Not all types can be stored in a WeakMap
Module.STDWEB_PRIVATE.ref_to_id_map_fallback = new Map();
Module.STDWEB_PRIVATE.last_refid = 1;

Module.STDWEB_PRIVATE.id_to_raw_value_map = {};
Module.STDWEB_PRIVATE.last_raw_value_id = 1;

Module.STDWEB_PRIVATE.acquire_rust_reference = function( reference ) {
    if( reference === undefined || reference === null ) {
        return 0;
    }

    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
    var ref_to_id_map = Module.STDWEB_PRIVATE.ref_to_id_map;
    var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;

    var refid = ref_to_id_map.get( reference );
    if( refid === undefined ) {
        refid = ref_to_id_map_fallback.get( reference );
    }
    if( refid === undefined ) {
        refid = Module.STDWEB_PRIVATE.last_refid++;
        try {
            ref_to_id_map.set( reference, refid );
        } catch (e) {
            ref_to_id_map_fallback.set( reference, refid );
        }
    }

    if( refid in id_to_ref_map ) {
        id_to_refcount_map[ refid ]++;
    } else {
        id_to_ref_map[ refid ] = reference;
        id_to_refcount_map[ refid ] = 1;
    }

    return refid;
};

Module.STDWEB_PRIVATE.acquire_js_reference = function( refid ) {
    return Module.STDWEB_PRIVATE.id_to_ref_map[ refid ];
};

Module.STDWEB_PRIVATE.increment_refcount = function( refid ) {
    Module.STDWEB_PRIVATE.id_to_refcount_map[ refid ]++;
};

Module.STDWEB_PRIVATE.decrement_refcount = function( refid ) {
    var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;
    if( 0 == --id_to_refcount_map[ refid ] ) {
        var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;
        var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;
        var reference = id_to_ref_map[ refid ];
        delete id_to_ref_map[ refid ];
        delete id_to_refcount_map[ refid ];
        ref_to_id_map_fallback.delete(reference);
    }
};

Module.STDWEB_PRIVATE.register_raw_value = function( value ) {
    var id = Module.STDWEB_PRIVATE.last_raw_value_id++;
    Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ] = value;
    return id;
};

Module.STDWEB_PRIVATE.unregister_raw_value = function( id ) {
    delete Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.get_raw_value = function( id ) {
    return Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ];
};

Module.STDWEB_PRIVATE.alloc = function alloc( size ) {
    return Module.web_malloc( size );
};

Module.STDWEB_PRIVATE.dyncall = function( signature, ptr, args ) {
    return Module.web_table.get( ptr ).apply( null, args );
};

// This is based on code from Emscripten's preamble.js.
Module.STDWEB_PRIVATE.utf8_len = function utf8_len( str ) {
    var len = 0;
    for( var i = 0; i < str.length; ++i ) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        var u = str.charCodeAt( i ); // possibly a lead surrogate
        if( u >= 0xD800 && u <= 0xDFFF ) {
            u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);
        }

        if( u <= 0x7F ) {
            ++len;
        } else if( u <= 0x7FF ) {
            len += 2;
        } else if( u <= 0xFFFF ) {
            len += 3;
        } else if( u <= 0x1FFFFF ) {
            len += 4;
        } else if( u <= 0x3FFFFFF ) {
            len += 5;
        } else {
            len += 6;
        }
    }
    return len;
};

Module.STDWEB_PRIVATE.prepare_any_arg = function( value ) {
    var arg = Module.STDWEB_PRIVATE.alloc( 16 );
    Module.STDWEB_PRIVATE.from_js( arg, value );
    return arg;
};

Module.STDWEB_PRIVATE.acquire_tmp = function( dummy ) {
    var value = Module.STDWEB_PRIVATE.tmp;
    Module.STDWEB_PRIVATE.tmp = null;
    return value;
};



    var HEAP8 = null;
    var HEAP16 = null;
    var HEAP32 = null;
    var HEAPU8 = null;
    var HEAPU16 = null;
    var HEAPU32 = null;
    var HEAPF32 = null;
    var HEAPF64 = null;

    Object.defineProperty( Module, 'exports', { value: {} } );

    function __web_on_grow() {
        var buffer = Module.instance.exports.memory.buffer;
        HEAP8 = new Int8Array( buffer );
        HEAP16 = new Int16Array( buffer );
        HEAP32 = new Int32Array( buffer );
        HEAPU8 = new Uint8Array( buffer );
        HEAPU16 = new Uint16Array( buffer );
        HEAPU32 = new Uint32Array( buffer );
        HEAPF32 = new Float32Array( buffer );
        HEAPF64 = new Float64Array( buffer );
        Module.HEAP8 = HEAP8;
        Module.HEAP16 = HEAP16;
        Module.HEAP32 = HEAP32;
        Module.HEAPU8 = HEAPU8;
        Module.HEAPU16 = HEAPU16;
        Module.HEAPU32 = HEAPU32;
        Module.HEAPF32 = HEAPF32;
        Module.HEAPF64 = HEAPF64;
    }

    return {
        imports: {
            env: {
                "__cargo_web_snippet_0548409232e47863a42b597d3edba1463d7ec2c2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).connected;})());
            },
            "__cargo_web_snippet_05eb1fa3a369aeb53fa30b81d7ffa84d99189bab": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).createBuffer();})());
            },
            "__cargo_web_snippet_07f749b6b9af4fd45ecb5ef8760b67d3c6aafcc5": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).onwheel=function(e){($1)(e.deltaX,e.deltaY,e.deltaMode);e.preventDefault();}
            },
            "__cargo_web_snippet_07f76243138bcc8dfb8a07a50088a3eb39f8ba2c": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getContext("webgl2");})());
            },
            "__cargo_web_snippet_09675c7ed2827e045dc760aeac3d286437cfbe5e": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return{value:function(){return($1).setAttribute(($2),($3));}(),success:true};}catch(error){return{error:error,success:false};}})());
            },
            "__cargo_web_snippet_0a6ddb433f3bd369cfb4cf3832d4ed8b8c02a6bc": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Date) | 0;
            },
            "__cargo_web_snippet_0e54fd9c163fcf648ce0a395fde4500fd167a40b": function($0) {
                var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "InvalidCharacterError");
            },
            "__cargo_web_snippet_0ee9cdf28602e40bb337ce7d02d7e745b6693177": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).enable(($1));
            },
            "__cargo_web_snippet_0f503de1d61309643e0e13a7871406891e3691c9": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return window;})());
            },
            "__cargo_web_snippet_0f74d910086cf797f181337a32d9a1219d19636b": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).enableVertexAttribArray(($1));
            },
            "__cargo_web_snippet_10f5aa3985855124ab83b21d4e9f7297eb496508": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Array) | 0;
            },
            "__cargo_web_snippet_133dc76bb1b71c4c01bb36ad66f6e9e9e56d7c29": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).head;})());
            },
            "__cargo_web_snippet_16a29a24997792c4d65aabe89033fb5dcb06b826": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).drawElements(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_1c121c2e0458f104d1e955b4252b1cb025c1686d": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof WebGL2RenderingContext) | 0;
            },
            "__cargo_web_snippet_1d215a652e33885c8a0c73fed6f412252bea75b5": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);($0).texParameteri(($1),($2),($3));
            },
            "__cargo_web_snippet_1e65287b40ff2503a5bd21bba8369d5759ddb2d4": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).height=($1);
            },
            "__cargo_web_snippet_1f59df591f2ed7b1c8923d281ff5495e8fb01bd5": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).axes;})());
            },
            "__cargo_web_snippet_2034053b7b6771271a2dad3e5416b045a74488a1": function($0, $1, $2, $3) {
                Module.STDWEB_PRIVATE.acquire_js_reference( $0 ).setTimeout( function() {Module.STDWEB_PRIVATE.dyncall( 'vi', $1, [$2] );}, $3 );
            },
            "__cargo_web_snippet_23639371cb88eaf0e4e3ff14ba63d1e5b5cea0b2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).key;})());
            },
            "__cargo_web_snippet_26251eb14c9e5da41a8d2a2c777d212d7b2a5304": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).id;})());
            },
            "__cargo_web_snippet_275c52510376b526efc3b77789bb01b8a440efd4": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).width;})());
            },
            "__cargo_web_snippet_29f46501f14cc29940baea6369862c876af72b70": function($0, $1, $2, $3, $4, $5, $6) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);$5 = Module.STDWEB_PRIVATE.to_js($5);$6 = Module.STDWEB_PRIVATE.to_js($6);($0).vertexAttribPointer(($1),($2),($3),($4),($5),($6));
            },
            "__cargo_web_snippet_2aa86295ceaf75cec3455bf6b35b97ed2221c3fb": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).offsetX;})());
            },
            "__cargo_web_snippet_2bbb15db388b3c1a320bdeaadf136327690bf341": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).deleteTexture(($1));
            },
            "__cargo_web_snippet_2e92e77c21dea6f3b01f9ba7ab22b0a1dae4a9ab": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).button;})());
            },
            "__cargo_web_snippet_352943ae98b2eeb817e36305c3531d61c7e1a52b": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Element) | 0;
            },
            "__cargo_web_snippet_37f123361e40b110cef478a4c4ab6d5b8c7e60cf": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).buffer;})());
            },
            "__cargo_web_snippet_3acb61e26a58d72c09ba72199a1768f2ed19718d": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).title=($1);
            },
            "__cargo_web_snippet_41648715735c41dd07f9e6ecd29694c149a1147f": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).bindTexture(($1),($2));
            },
            "__cargo_web_snippet_4fd31c9e56d40b8642cf9e6f96fd6b570f355cea": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);console.error(($0));
            },
            "__cargo_web_snippet_51c698c3896cefa6badebc5a5c16586cf31af567": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).bindBuffer(($1),($2));
            },
            "__cargo_web_snippet_522594a9ab8b9160e7c4bc14704529f31eacd977": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getTime();})());
            },
            "__cargo_web_snippet_53e1733171846cd3cdb5ae4eebb597c29d388696": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof GamepadEvent && o.type === "gamepadconnected") | 0;
            },
            "__cargo_web_snippet_5984245de8b6ef88f693ba2383ebf3c2f9718c6c": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof HTMLCanvasElement) | 0;
            },
            "__cargo_web_snippet_5c57e16ebd22655f976d87fae8039e282d7bab59": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return Array.from(navigator.getGamepads());})());
            },
            "__cargo_web_snippet_5d1dea1aed3b2e8d1e8b9c1dd7cab175f40425e7": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).index;})());
            },
            "__cargo_web_snippet_5ff24e9704111df2c482ac39341902174ab3c66c": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).offsetY;})());
            },
            "__cargo_web_snippet_60a4678a86465066d08a972378bb333945f797a6": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof GamepadEvent && o.type === "gamepaddisconnected") | 0;
            },
            "__cargo_web_snippet_613b0f25af124f7a157482b1898be5a8759ccb97": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof PointerEvent && o.type === "pointerover") | 0;
            },
            "__cargo_web_snippet_614a3dd2adb7e9eac4a0ec6e59d37f87e0521c3b": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).error;})());
            },
            "__cargo_web_snippet_652a44a9a9c25aa377162e42c1c0ca2b47b621e6": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).buttons;})());
            },
            "__cargo_web_snippet_6895877ca617093d48433651d9853b2907713750": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).clearColor(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_6a0bf07b6e92fa20cf6af04918b9707e6ebc23bf": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof ArrayBuffer) | 0;
            },
            "__cargo_web_snippet_6bdb77327abaf05b05161715fc6ed5f75d3875b8": function($0) {
                return (Module.STDWEB_PRIVATE.acquire_js_reference( $0 ) instanceof Uint32Array) | 0;
            },
            "__cargo_web_snippet_6f40eea589df25151032f882dd63be9d49c014a4": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).deleteBuffer(($1));
            },
            "__cargo_web_snippet_6fcce0aae651e2d748e085ff1f800f87625ff8c8": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return document;})());
            },
            "__cargo_web_snippet_713d1b04d9e622498f19dae5286acec13a6b2a12": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof WebGLBuffer) | 0;
            },
            "__cargo_web_snippet_734a82ee4ac42c380518cb72c7adb54b7c35c8fe": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).linkProgram(($1));
            },
            "__cargo_web_snippet_76b04ba9e32018da808d8aa674d3f0898fe28500": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof FocusEvent && o.type === "blur") | 0;
            },
            "__cargo_web_snippet_76c1ee6b34a4c89a8e2052fb46de66eee5144608": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof KeyboardEvent && o.type === "keyup") | 0;
            },
            "__cargo_web_snippet_77e0e1ec1d4bd4ae51076c4c84da5c7214096ebe": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).clear(($1));
            },
            "__cargo_web_snippet_7b0825ae89bed906bbdd29f8ee2ceb22c4fef516": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).width=($1);
            },
            "__cargo_web_snippet_7ba9f102925446c90affc984f921f414615e07dd": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).body;})());
            },
            "__cargo_web_snippet_7bead6b563d52eee65504adb6b76c5cacb5428d3": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).preventDefault();
            },
            "__cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0": function($0) {
                Module.STDWEB_PRIVATE.decrement_refcount( $0 );
            },
            "__cargo_web_snippet_85b9ecbdb8513465b790546acfd0cd530441b8a4": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);($0).stopPropagation();
            },
            "__cargo_web_snippet_8956c802ab8a6863a8347558d363df26d3d2b9ab": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).useProgram(($1));
            },
            "__cargo_web_snippet_8ad5e488b61134cc6d61a42368f5146234bb0e68": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).attachShader(($1),($2));
            },
            "__cargo_web_snippet_8c32019649bb581b1b742eeedfc410e2bedd56a6": function($0, $1) {
                var array = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );Module.STDWEB_PRIVATE.serialize_array( $1, array );
            },
            "__cargo_web_snippet_8ec946091bb781e760245e0ae35985deddcdbb25": function($0) {
                return (Module.STDWEB_PRIVATE.acquire_js_reference( $0 ) instanceof Float32Array) | 0;
            },
            "__cargo_web_snippet_91749aeb589cd0f9b17cbc01b2872ba709817982": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return{value:function(){return($1).createElement(($2));}(),success:true};}catch(error){return{error:error,success:false};}})());
            },
            "__cargo_web_snippet_931b21f9ef477fd3f89116c8eca87dcc2964fabb": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);($0).bufferSubData(($1),($2),($3));
            },
            "__cargo_web_snippet_93677063bfe06049229cc979f161294aa83b0bfe": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof PointerEvent && o.type === "pointerup") | 0;
            },
            "__cargo_web_snippet_975aedb4ce06afaf5f5db053987b8e3e8007c4f6": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).mapping;})());
            },
            "__cargo_web_snippet_99c4eefdc8d4cc724135163b8c8665a1f3de99e4": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var listener=($1);($2).addEventListener(($3),listener);return listener;})());
            },
            "__cargo_web_snippet_9a7eca60ef721cc6257c116658f2fcdd49070cd1": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return new Date();})());
            },
            "__cargo_web_snippet_9ad57da784f7eff4114206c1b4c14a82c36558fc": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).createProgram();})());
            },
            "__cargo_web_snippet_9bc28cbc8b224706449edaedfb4361572acd03fa": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).gamepad;})());
            },
            "__cargo_web_snippet_9c1932d56bd62874949fd35c248149241e00cda2": function($0, $1) {
                return Module.STDWEB_PRIVATE.acquire_rust_reference( Module.HEAPU32.slice( $0, $1 ) );
            },
            "__cargo_web_snippet_9cbe3f041910b7b8710cf6692e325e3c397db838": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof PointerEvent && o.type === "pointermove") | 0;
            },
            "__cargo_web_snippet_9d64a695070c583ca1db88f92170810d90b0bb4c": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof KeyboardEvent && o.type === "keydown") | 0;
            },
            "__cargo_web_snippet_9ea12c743a87885cb0ca7f3ffa97fe45e368cda7": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getAttribLocation(($2),($3));})());
            },
            "__cargo_web_snippet_9f22d4ca7bc938409787341b7db181f8dd41e6df": function($0) {
                Module.STDWEB_PRIVATE.increment_refcount( $0 );
            },
            "__cargo_web_snippet_a08318f2d6ae2d01fde542f3519007735c4915d2": function($0, $1) {
                return Module.STDWEB_PRIVATE.acquire_rust_reference( Module.HEAPF32.slice( $0, $1 ) );
            },
            "__cargo_web_snippet_a3cb955a4adee2af3bd3bde79bfe035414e04a10": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).deleteProgram(($1));
            },
            "__cargo_web_snippet_a40cb09bd17fb2733e9d31b72e85b296d8b69c65": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).uniform1i(($1),($2));
            },
            "__cargo_web_snippet_a6869298bc27aad2e9079e7c1c03cbc00bfe61fe": function($0, $1, $2, $3) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);($0).bufferData(($1),($2),($3));
            },
            "__cargo_web_snippet_a8869cf2d77ba584443739645e4f4b714afb506f": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).viewport(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_ab05f53189dacccf2d365ad26daa407d4f7abea9": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).value;})());
            },
            "__cargo_web_snippet_ab5cb3942c38455c95c95123489758fc220d9565": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof PointerEvent && o.type === "pointerdown") | 0;
            },
            "__cargo_web_snippet_ac7d2ed6b6f9c8022b4938c48fabf184cdb62103": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).compileShader(($1));
            },
            "__cargo_web_snippet_aced5a0bc31e9d4b2c0a70c656f71a6a7334963a": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof WebGLProgram) | 0;
            },
            "__cargo_web_snippet_b06dde4acf09433b5190a4b001259fe5d4abcbc2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).success;})());
            },
            "__cargo_web_snippet_b48186ca559ce6b79488859686e2529ec3c9f674": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).deleteShader(($1));
            },
            "__cargo_web_snippet_b4ec7b590e46107bd51f42bef46039f771ee5e74": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);window.onclose=($0);
            },
            "__cargo_web_snippet_b5279855d56d8dbb2124ea9f036133d0f76408de": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof FocusEvent && o.type === "focus") | 0;
            },
            "__cargo_web_snippet_b6fbe111e441333398599f63dc09b26f8d172654": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).innerHeight;})());
            },
            "__cargo_web_snippet_b71edda6abdc82b7ba10e4d557888d8458548a28": function($0, $1, $2, $3, $4) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);$4 = Module.STDWEB_PRIVATE.to_js($4);($0).blendFuncSeparate(($1),($2),($3),($4));
            },
            "__cargo_web_snippet_c3f167cac56fb49380b4761631eaa17dba921f1d": function($0, $1, $2) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).createShader(($2));})());
            },
            "__cargo_web_snippet_c5832a87376bd02b13fd6a2003d3562c36a4bb00": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).cancelBubble;})());
            },
            "__cargo_web_snippet_cbffc577197fc32adfecb64174d9e3f6ea25736b": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).style.cursor=($1)
            },
            "__cargo_web_snippet_d2e4181d99d09b8dcdaf227704c44b7be437abd4": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Event && o.type === "resize") | 0;
            },
            "__cargo_web_snippet_d3336fefc8646aa17b501ca0d1fc23db2bfd8df2": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).height;})());
            },
            "__cargo_web_snippet_d3a449492a1f92bb689edccf2aacd868ad253fc0": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Gamepad) | 0;
            },
            "__cargo_web_snippet_d5268550db8834dda9a5cb31d532c54071c704a2": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).activeTexture(($1));
            },
            "__cargo_web_snippet_d583a887a73b8c2645110ea43ab0a2fcb28d2b3d": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).getUniformLocation(($2),($3));})());
            },
            "__cargo_web_snippet_d5a793fae1267a877b445337cdc5e8d5d429a9d0": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof WebGLUniformLocation) | 0;
            },
            "__cargo_web_snippet_d6d59a188b8825cd58baaa4a29c4a918ec7cb003": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof WebGLShader) | 0;
            },
            "__cargo_web_snippet_e313f15435acbda1ee50355801d6dd989604ca8d": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof PointerEvent && o.type === "pointerout") | 0;
            },
            "__cargo_web_snippet_e741b9d9071097746386b2c2ec044a2bc73e688c": function($0, $1) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).appendChild(($1));
            },
            "__cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec": function($0) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);Module.STDWEB_PRIVATE.unregister_raw_value(($0));
            },
            "__cargo_web_snippet_ea6ad9d8415e84119621f5aa2c86a39abc588b75": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).innerWidth;})());
            },
            "__cargo_web_snippet_eb7adec904fbe623da3e9c5ca4762c710c3029be": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).blendEquationSeparate(($1),($2));
            },
            "__cargo_web_snippet_eba9d12fc35ac2dfa6e24f4687d4d4639298f1ce": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).pressed;})());
            },
            "__cargo_web_snippet_eda4412440e7d32f8fe74d951b52da51099b5d05": function($0, $1, $2) {
                $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);($0).shaderSource(($1),($2));
            },
            "__cargo_web_snippet_ee2ac14eaf4087d775bf8366e5a938975a26a075": function($0) {
                var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof GamepadButton) | 0;
            },
            "__cargo_web_snippet_f1c5b555b7858c4f021b91769dce6f5bafdef9a2": function($0, $1, $2, $3) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var callback=($1);var request=($2).requestAnimationFrame(callback);return{request:request,callback:callback,window:($3)};})());
            },
            "__cargo_web_snippet_f91e3759ea8e40c15404c8b089c74fe76b61421e": function($0) {
                Module.STDWEB_PRIVATE.from_js($0, (function(){return Date.now();})());
            },
            "__cargo_web_snippet_fc18467ee7b1f9a5e6e811e07f6fff2c88679ff3": function($0, $1) {
                $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return($1).code;})());
            },
            "__cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0": function($0) {
                Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js( $0 );
            },
                "__web_on_grow": __web_on_grow
            }
        },
        initialize: function( instance ) {
            Object.defineProperty( Module, 'instance', { value: instance } );
            Object.defineProperty( Module, 'web_malloc', { value: Module.instance.exports.__web_malloc } );
            Object.defineProperty( Module, 'web_free', { value: Module.instance.exports.__web_free } );
            Object.defineProperty( Module, 'web_table', { value: Module.instance.exports.__indirect_function_table } );

            
            __web_on_grow();
            Module.instance.exports.main();

            return Module.exports;
        }
    };
}
 ));
}));
