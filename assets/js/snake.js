//Globale Variablen

//Koordinaten der Schlange, der erste Eintrag ist der Kopf der Schlange
var snake = [{ x: 3, y: 2 }, { x: 4, y: 2 }, { x: 5, y: 2 }];
//Koordinaten der Frucht
var fruit = { x: 5, y: 5 };

//direction vecs
var links = { x: -1, y: 0 };
var rechts = { x: 1, y: 0 };
var oben = { x: 0, y: -1 };
var unten = { x: 0, y: 1 };

//Richtung, in der sich die Schlange momentan bewegt
var currentDirection = links;

//Größe der Zelle
var snakeCell = 20;

//initiale Punkte
var punkte = 0;

//indicator if game is running
var snakeGameRun = false;

//Spielstand zeichnen:

var snakeCanvas = document.getElementById('snake-canvas');
var sctx = snakeCanvas.getContext('2d');

snakeCanvas.style.backgroundColor = "#D3D3D3";
snakeCanvas.style.borderRadius = "10px";

function drawSnakeMenu() {
    sctx.fillStyle = "black";
    sctx.fillRect(0, 0, snakeCanvas.width, snakeCanvas.width);

    let title = "SNAKE";
    sctx.fillStyle = 'white';
    sctx.textAlign = 'center';
    sctx.font = '50px Arial';
    sctx.fillText(title, snakeCanvas.width / 2, snakeCanvas.height / 2);

    drawSnake([
            {x: 10, y: 15}, 
            {x: 11, y: 15}, 
            {x: 12, y: 15}
        ]);
    drawFruit(fruit);
}

function drawFruit(fruit) {
    sctx.lineWidth = 2;
    sctx.strokeStyle = "#D3D3D3";
    sctx.fillStyle = "#538700";
    sctx.fillRect(fruit.x * snakeCell, fruit.y * snakeCell, snakeCell, snakeCell);
    sctx.strokeRect(fruit.x * snakeCell, fruit.y * snakeCell, snakeCell, snakeCell);

}

function drawSnake(snake) {
    for (let i = 0; i < snake.length; i++) {

        sctx.lineWidth = 2;
        sctx.strokeStyle = "#D3D3D3";
        if (i == 0) {
            sctx.fillStyle = "#000000";
        } else {
            sctx.fillStyle = "#00538E";
        }
        sctx.fillRect(snake[i].x * snakeCell, snake[i].y * snakeCell, snakeCell, snakeCell);
        sctx.strokeRect(snake[i].x * snakeCell, snake[i].y * snakeCell, snakeCell, snakeCell);
    }


}


function drawGameOver(punkte) {
    text1 = "Game Over! \n";
    text2 = punkte + " Punkte";

    sctx.fillStyle = 'white';
    sctx.textAlign = 'center';
    sctx.font = '50px Arial';
    sctx.fillText(text1, snakeCanvas.width / 2, snakeCanvas.height / 2);
    sctx.font = '25px Arial';
    sctx.fillText(text2, snakeCanvas.width / 2, (snakeCanvas.height / 2) + 50);
}

//Spiellogik

function fruitCollidesWithSnake(snake, fruit) {
    var collide = false;

    for (let i = 0; i < snake.length; i++) {
        if ((snake[i].x == fruit.x) && (snake[i].y == fruit.y)) {
            collide = true;
            break;
        }
    }
    return collide;
}

function randomCoordinateOutsideSnake(snake) {

    var zufallX = Math.round(Math.random() * 19);
    var zufallY = Math.round(Math.random() * 19);

    var zufall = { x: zufallX, y: zufallY }

    while (fruitCollidesWithSnake(snake, zufall)) {
        zufallX = Math.round(Math.random() * 19);
        zufallY = Math.round(Math.random() * 19);
        zufall = { x: zufallX, y: zufallY }
    }

    return zufall;
}

function snakeHeadCollidesWithSnake(snake) {
    var collide = false;
    for (var i = 1; i < snake.length; i++) {
        if ((snake[0].x == snake[i].x) && (snake[0].y == snake[i].y)) {
            collide = true;
            break;
        }
    }
    return collide;
}

function moveSnake(snake, directionVector) {

    var pastTail = snake.pop();
    var newHead = { x: snake[0].x + directionVector.x, y: snake[0].y + directionVector.y };

    if (newHead.x > 19 || newHead.y > 19 || newHead.x < 0 || newHead.y < 0) {
        newHead = { x: mod(snake[0].x + directionVector.x, snakeCell), y: mod(snake[0].y + directionVector.y, snakeCell) };
    }
    snake.unshift(newHead);
    return pastTail;

}

function mod(n, m) {
    return ((n % m) + m) % m;
}

function initialSetup() {
    snake = [{ x: 3, y: 2 }, { x: 4, y: 2 }, { x: 5, y: 2 }];
    fruit = { x: 5, y: 5 };
    currentDirection = links;
    punkte = 0;
    drawSnake(snake);
    drawFruit(fruit);
}

//Game-und Animation-Loop
function startSnakeGame() {
    var intervalID = setInterval(function () {
        //Bewegen und zeichnen der Schlange

        var tail = moveSnake(snake, currentDirection);

        if (fruitCollidesWithSnake(snake, fruit)) {
            fruit = randomCoordinateOutsideSnake(snake);
            snake.push(tail);
        }
        sctx.clearRect(0, 0, snakeCanvas.width, snakeCanvas.height);
        drawSnake(snake);
        drawFruit(fruit);


        //wenn das Spiel beendet ist, muss setInterval gestoppt werden:
        if (snakeHeadCollidesWithSnake(snake)) {
            punkte = snake.length - 3;
            drawGameOver(punkte);
            clearInterval(intervalID);
            snakeGameRun = false;
        }
    }, 150);
}

//Key-Down listener
document.body.addEventListener('keydown', function (event) {

    if ((event.key == "ArrowRight") && (currentDirection != links)) {

        currentDirection = rechts;
    }
    if ((event.key == "ArrowLeft") && (currentDirection != rechts)) {

        currentDirection = links;
    }

    if ((event.key == "ArrowUp") && (currentDirection != unten)) {

        currentDirection = oben;
    }

    if ((event.key == "ArrowDown") && (currentDirection != oben)) {

        currentDirection = unten;
    }

});

snakeCanvas.addEventListener('mousedown', function (event) {
    if (!snakeGameRun) {
        initialSetup();
        startSnakeGame();
        snakeGameRun = true;
    }
});

drawSnakeMenu();