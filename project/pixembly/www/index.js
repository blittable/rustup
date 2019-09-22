import * as wasm from "wasm-pixembly";

function handleFileSelect(evt) {
    var reader = new FileReader();
    reader.onloadend = function (evt) {
        if (evt.target.readyState == FileReader.DONE) { // DONE == 2
            let canvas = document.getElementById('canvas');
            let ctx = canvas.getContext('2d');

            var img = new Image();
            img.onload = function () {
                canvas.width = img.width;
                canvas.height = img.height;
                ctx.drawImage(img, 0, 0);
            }
            img.src = evt.target.result;
        }
    };
    reader.readAsDataURL(evt.target.files[0]);
}
document
    .getElementById('files')
    .addEventListener('change', handleFileSelect, false);

document
    .querySelectorAll('button').forEach(function(e) {
        e.addEventListener('click', function() {
            let canvas = document.getElementById('canvas');
            let encoded = canvas.toDataURL('image/bmp').replace(/^data:(.*,)?/, '');;
            if ((encoded.length % 4) > 0) {
                encoded += '='.repeat(4 - (encoded.length % 4));
            }
            wasm.load_image("canvas", encoded, this.innerHTML);
        }, false);
    });
    