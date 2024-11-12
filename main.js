import init, { Coordinate, MyBotDetection } from "./pkg/cursed_rust.js"

await init();

// Create a Rust object
let detection = new MyBotDetection();
// Other state
/** @type { import("./pkg/cursed_rust.js").Event[] } */
let allEvents = [];
/** @type { BotDetectionOutput | null } */
let latestResult = null;

// DOM references
const counterElement = document.getElementById('counter');
const humanScoreElement = document.getElementById('human-score');
const botResultElement = document.getElementById('bot-result');
const previousResultsElement = document.getElementById('previous-results');
const debugInfo = document.getElementById('debug-info');

/** Keep DOM up to date with state. */
function update() {
    if (latestResult) {
        humanScoreElement.innerText = `Human Score: ${latestResult.humanScore.toFixed(3)}`;
        botResultElement.innerText = latestResult.text();
    }

    counterElement.innerText = `Events generated: ${allEvents.length}`;

    previousResultsElement.innerHTML = "";
    let results = detection.results;
    for (let i = 0; i < results.length; i++) {
        // create DOM elements
        const row = document.createElement("div")
        const firstDiv = document.createElement("div")
        const secondDiv = document.createElement("div")

        // set CSS classes
        row.classList.add("result-row");
        firstDiv.classList.add("small");
        secondDiv.classList.add("small");

        // set content
        secondDiv.innerText = results[i].text();
        firstDiv.innerText = (new Date(results[i].timestamp)).toUTCString();

        row.appendChild(firstDiv);
        row.appendChild(secondDiv);
        previousResultsElement.appendChild(row);
    }

    requestAnimationFrame(showErr(update));
}
requestAnimationFrame(showErr(update));

/* On every mouse movement, record another event. */
window.onmousemove = (event) => {
    detection.addEvent(Date.now(), event);
    allEvents = detection.events;
};

function amIBot() {
    latestResult = detection.isBot();
    detection.saveResult(latestResult);
}

function printWindowedJitter() {
    let windowSize = 100;
    let window;
    if (allEvents.length <= windowSize) {
        window = allEvents;
    } else {
        window = allEvents.slice(-windowSize);
    }

    const tmp = MyBotDetection.fromEvents(window);
    debugInfo.innerText = tmp.isBot().humanScore.toFixed(3);
}

function setXtoZero() {
    for (let i = 0; i < allEvents.length; i++) {
        allEvents[i].coordinate.x = 0;
    }
    printWindowedJitter();
}

document.getElementById('button1').onclick = showErr(amIBot);
document.getElementById('button2').onclick = showErr(printWindowedJitter);
document.getElementById('button3').onclick = showErr(setXtoZero);

function showErr(func) {
    return () => {
        try {
            func();
        } catch (error) {
            const message = `${error.message || error.toString()}`;
            document.body.innerHTML = `<div class="panic">RUST PANIC:<p>${message}</p></div>`;
            console.error(error);
        }
    };
}
