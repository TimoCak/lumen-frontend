var canvas = document.getElementById("canvas");
canvas.style.backgroundColor = "#D3D3D3";
canvas.style.borderRadius = "10px";

var ctx = canvas.getContext('2d');

/*
CANVAS-IMAGES
*/
var background_image = new Image();
var mage_image = new Image();
var zombie_image = new Image();

function loadImages() {
    background_image.src = "/assets/images/forest.png";
    mage_image.src = "/assets/images/magician.png";
    zombie_image.src = "/assets/images/zombie_cutted.png";
}

var gradient = ctx.createLinearGradient(0, 0, canvas.width, 0);
gradient.addColorStop("0", "magenta");
gradient.addColorStop("0.5", "blue");
gradient.addColorStop("1.0", "red");

var bulletSize = 5;
var mageSize = 50;
var zombieSize = 50;

var points = 0;
var run = false;
var combo = 1;

var magicianCoord = { x: canvas.width / 2, y: canvas.height / 2 };

//direction vecs
var left = { x: -10, y: 0 };
var right = { x: 10, y: 0 };
var up = { x: 0, y: -10 };
var down = { x: 0, y: 10 };

var bullets = [];
var zombies = [];
var dialog = ["Weichet von mir!", "Die sind so gut wie tot!", "Ich bin unsterblich!", "Schon einmal gegen ein Zauberer gekämpf?"];

async function drawMainMenu() {
    await drawBackgroundImage();

    setTimeout(() => {
        ctx.fillStyle = "black";
        ctx.fillRect(canvas.width / 4 - 75, 0, 400, 200);
    }, 500)

    await drawMenuMagician();

    setTimeout(() => {
        ctx.fillStyle = gradient;
        ctx.textAlign = "center";
        ctx.font = '30px Arial';
        ctx.fillText("Welcome Wizard!", canvas.width / 2, 50);
    }, 500)

    setTimeout(() => {
        ctx.font = '20px Arial';
        ctx.fillText("Zombies invading this Website!", canvas.width / 2, 100);
        ctx.fillText("Kill them with your magical power!", canvas.width / 2, 150);
    }, 500)
}

async function drawMenuMagician() {
    setTimeout(() => {
        ctx.drawImage(mage_image, canvas.width / 2 - 200, canvas.height / 2 - 50);
    }, 500)
}

async function drawBackgroundImage() {
    setTimeout(() => {
        ctx.drawImage(background_image, 0, 0, canvas.width, canvas.height);
    }, 500)
}

function drawMagician() {
    ctx.drawImage(mage_image, magicianCoord.x, magicianCoord.y, mageSize, mageSize);
}

function drawAndMoveBullets() {
    let counter = 0;
    bullets.forEach((bullet) => {
        ctx.beginPath();
        ctx.arc(bullet.x, bullet.y, bulletSize, 0, 2 * Math.PI);
        ctx.fillStyle = "orange";
        ctx.strokeStyle = "orange";
        ctx.fill();
        bullet.x += right.x * 2;
        bullet.y += right.y * 2;

        if (bullet.x > canvas.width) {
            despawnBullet(counter);
        }
        counter++;
    });
}

function drawDialog(randomIndex) {
    ctx.fillStyle = gradient;
    ctx.font = '15px Arial';
    ctx.fillText(dialog[randomIndex], magicianCoord.x, magicianCoord.y);
}

function drawAndMoveZombies() {
    zombies.forEach((zombie) => {
        ctx.drawImage(zombie_image, zombie.x, zombie.y, zombieSize, zombieSize);
        zombie.x += left.x / 4;
    });
}

function drawPoints() {
    ctx.fillStyle = gradient;
    ctx.font = '20px Arial';
    ctx.fillText("Score: " + points, canvas.width / 2, 25);
}

function drawAndEnableCombo() {
    let comboText = "Combo: ";
    ctx.fillStyle = gradient;
    ctx.font = '20px Arial';
    if (points > 100) {
        combo = 4;
        ctx.fillText(comboText + combo + "x", 60, 25);
        ctx.fillText("FICK SIE !!!", 60, 50);
    }
    else if (points > 50) {
        combo = 3;
        ctx.fillText(comboText + combo + "x", 60, 25);
    }
    else if (points > 10) {
        combo = 2;
        ctx.fillText(comboText + combo + "x", 60, 25);
    } else {
        combo = 1;
    }
}

function despawnZombie(index) {
    zombies.splice(index, 1);
}

function despawnBullet(index) {
    bullets.splice(index, 1);
}

function BulletZombieCollision() {
    bullets.forEach((bullet) => {
        for (let i = 0; i < zombies.length; i++) {
            if (
                bullet.x < zombies[i].x + zombieSize &&
                bullet.x + bulletSize > zombies[i].x &&
                bullet.y < zombies[i].y + zombieSize &&
                bullet.y + bulletSize > zombies[i].y

            ) {

                points += combo;
                despawnBullet(i);
                despawnZombie(i);
            }
        }
    });
}

function ZombieOutOfMapCollision() {
    for (let i = 0; i < zombies.length; i++) {
        if (zombies[i].x + zombieSize < 0) {
            despawnZombie(i);
            points--;
        }
    }
}

function ZombieMageCollision(interval) {
    for (let i = 0; i < zombies.length; i++) {
        if (
            magicianCoord.x < zombies[i].x + zombieSize &&
            magicianCoord.x + mageSize > zombies[i].x &&
            magicianCoord.y < zombies[i].y + zombieSize &&
            magicianCoord.y + mageSize > zombies[i].y

        ) {
            //maybe endscreen
            clearInterval(interval);
            run = false;
            drawMainMenu();
        }
    }
}

function changeDifficulty() {
    let difficulty = 50;
    if (points > 100) {
        difficulty = 25;
    } else if (points > 50) {
        difficulty = 40;
    }
    let dice = Math.floor((Math.random() * difficulty) + 1)
    if (dice === difficulty) {
        zombies.push({ x: canvas.width, y: Math.random() * canvas.height });
    }
}
//game-loop
function startGame() {
    points = 0;
    combo = 1;
    bullets = [];
    zombies = [];

    let intervall = setInterval(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawMagician();
        drawAndMoveBullets();
        drawAndMoveZombies();
        drawPoints();
        drawAndEnableCombo();
        BulletZombieCollision();
        ZombieOutOfMapCollision();
        ZombieMageCollision(intervall);
        changeDifficulty();

        if (shootCounter % 10 === 0) {
            drawDialog(randomIndex);
        }

    }, 10);
}

var shootCounter = 0;
var randomIndex = Math.floor(Math.random() * dialog.length);

document.body.addEventListener('keydown', function (event) {
    //check if mage is out of canvas

    switch (event.key) {
        case "w":
            if (magicianCoord.y > 0) {
                magicianCoord.x += up.x;
                magicianCoord.y += up.y;
            }
            break;
        case "a":
            if (magicianCoord.x > 0) {
                magicianCoord.x += left.x;
                magicianCoord.y += left.y;
            }

            break;
        case "s":
            if (magicianCoord.y < canvas.height - 50) {
                magicianCoord.x += down.x;
                magicianCoord.y += down.y;
            }
            break;
        case "d":
            if (magicianCoord.x < canvas.width - 50) {
                magicianCoord.x += right.x;
                magicianCoord.y += right.y;
            }
            break;
        case "q":
            bullets.push({ x: magicianCoord.x + 50, y: magicianCoord.y + 20 });
        default:
            console.log("kein Holz my lord!");
    }
});

canvas.addEventListener('mousedown', function (event) {
    shootCounter++;
    randomIndex = Math.floor(Math.random() * dialog.length);
    if (run) {
        bullets.push({ x: magicianCoord.x + 50, y: magicianCoord.y + 20 });
    } else {
        startGame();
        run = true;
    }
});

function initialSetup() {
    loadImages();
    drawMainMenu();
}
initialSetup();