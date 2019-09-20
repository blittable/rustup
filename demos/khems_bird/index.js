import CatImage from "./static/bird.jpg";

var canvas;
var ctx;

import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('bird_container');
        const ctx = canvas.getContext('2d');

        //TODO
        //const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            wasm.transform_bird(ctx, 600, 600);
        });
    })
    .catch(console.error);


const newimg = new Image();
newimg.src = CatImage;
newimg.style.display = "none";

newimg.onload = () => {
    setUpCanvas();
}

function setUpCanvas() {
    let element = document.getElementById("image_container");
    element.appendChild(newimg);

    canvas = document.getElementById("bird_container");
    canvas.width = newimg.width;
    canvas.height = newimg.height;

    ctx = canvas.getContext("2d");
    ctx.drawImage(newimg, 0, 0);
}


