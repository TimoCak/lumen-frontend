let canvas = document.getElementById("canvas");
canvas.style.backgroundColor = "#D3D3D3";
canvas.style.borderRadius = "10px";

let ctx = canvas.getContext('2d');

const gradient = ctx.createLinearGradient(0, 0, canvas.width, 0);
gradient.addColorStop("0", "magenta");
gradient.addColorStop("0.5", "blue");
gradient.addColorStop("1.0", "red");

const bulletSize = 5;
const mageSize = 50;
const zombieSize = 50;

let points = 0;
let run = false;
let combo = 1;

let magicianCoord = { x: canvas.width / 2, y: canvas.height / 2 };

//direction vecs
const left = { x: -10, y: 0 };
const right = { x: 10, y: 0 };
const up = { x: 0, y: -10 };
const down = { x: 0, y: 10 };

let bullets = [];
let zombies = [];
let dialog = ["Weichet von mir!", "Die sind so gut wie tot!", "Ich bin unsterblich!", "Schon einmal gegen ein Zauberer gekämpf?"];

function drawMainMenu() {
    backgroundImage();
    ctx.fillStyle = "black";
    ctx.fillRect(canvas.width / 4 - 25 , 0, 300, 150);

    drawMenuMagician();

    ctx.fillStyle = gradient;
    ctx.textAlign = "center";
    ctx.font = '30px Arial';
    ctx.fillText("Welcome Wizard!", canvas.width / 2 , 50);

    ctx.font = '20px Arial';
    ctx.fillText("Kill them all!!! -__-", canvas.width / 2, 100);

}

function drawMenuMagician() {
    let mage_image = new Image();
    mage_image.src = "/assets/images/magician.png";
    ctx.drawImage(mage_image, canvas.width / 2 - 200, canvas.height / 2 - 50);
}

function backgroundImage() {
    let background_image = new Image();
    background_image.src = "/assets/images/forest.png"
    ctx.drawImage(background_image, 0, 0, canvas.width, canvas.height);
}

function drawMagician() {
    let mage_image = new Image();
    mage_image.src = "/assets/images/magician.png";
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
    let zombie_image = new Image();
    zombie_image.src = "/assets/images/zombie_cutted.png";
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
    let comboText = "Combo:";
    ctx.fillStyle = gradient;
    ctx.font = '20px Arial';
    if (points > 100) {
        combo = 4;
        ctx.fillText("Combo: " + combo + "x", 60, 25);
        ctx.fillText("FICK SIE !!!", 60, 50);
    }
    else if (points > 50) {
        combo = 3;
        ctx.fillText("Combo: " + combo + "x", 60, 25);
    }
    else if (points > 10) {
        combo = 2;
        ctx.fillText("Combo: " + combo + "x", 60, 25);
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

let shootCounter = 0;
let randomIndex = Math.floor(Math.random() * dialog.length);

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
                console.log(magicianCoord.y + " " + canvas.height);
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

document.body.addEventListener('mousedown', function (event) {
    shootCounter++;
    randomIndex = Math.floor(Math.random() * dialog.length);
    if (run) {
        bullets.push({ x: magicianCoord.x + 50, y: magicianCoord.y + 20 });
    } else {
        startGame();
        run = true;
    }
});

drawMainMenu();

