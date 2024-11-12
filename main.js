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

    /* PART 2 */
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
    /**/


    requestAnimationFrame(update);
    // For a live demo
    // requestAnimationFrame(showErr(update));
}
requestAnimationFrame(showErr(update));

/* On every mouse movement, record another event. */
window.onmousemove = (event) => {
    detection.addEvent(Date.now(), event);
    /* BUG 1, allEvents is undefined */
    /* BUG 1.1, events stuck at length 2, no error shows that events is not a function */
    allEvents = detection.events;
    // console.log("allEvents is", allEvents);

    // BUG 1.2, nothing happens, no error shows that events was called without parameters.
    // allEvents = detection.events();

    // solution
    // allEvents = detection.events(0, detection.num_events());
};



function amIBot() {
    latestResult = detection.isBot();

    /* BUG 2
     * Error: null pointer passed to rust
     */
    // console.log("latestResult before is", latestResult);
    detection.saveResult(latestResult);
    // console.log("latestResult after is", latestResult);

    /* Debugging? */
    // Error: recursive use of an object detected which would lead to unsafe aliasing in rust
    // detection.saveResult(latestResult);
    // detection.saveResult(latestResult);

    // solution
    // detection.saveBorrowedResult(latestResult);
}

function printWindowedJitter() {
    /* BUG 4, on second click
     * Error: array contains a value of the wrong type
     */

    let windowSize = 100;
    let window;
    if (allEvents.length <= windowSize) {
        window = allEvents;
    } else {
        window = allEvents.slice(-windowSize);
    }
    // BUG 4, debugging
    // This only prints a reference to the array, which when expanded, shows a bunch of null pointers.
    // console.log("window", window);

    // BUG 4, debugging continued
    // This shows the content at the time of execution.
    // for (let el of window) {
    //     // This shows a pointer, which when expanded still points to nothing.
    //     console.log(el);
    //     // This shows the values for real.
    //     console.log(el, el.coordinate.x, el.coordinate.y);
    // }

    const tmp = MyBotDetection.fromEvents(window);
    console.log("Human score of last", windowSize, "events was", tmp.isBot().humanScore);
    debugInfo.innerText = tmp.isBot().humanScore.toFixed(3);

    // solution BUG 4, restore allEvents
    // allEvents = detection.events(0, detection.num_events());
}

function setXtoZero() {
    for (let i = 0; i < allEvents.length; i++) {
        /* BUG 5
         * Copies the coordinates into a new Rc<WasmRefCell> and sets the value of
         * that copy, before dropping it again, leaving the original value unchanged.
         */
        // console.log("x was", allEvents[i].coordinate.x);
        allEvents[i].coordinate.x = 0;
        // console.log("x should be 0 now but actually is", allEvents[i].coordinate.x);

        // solution
        // allEvents[i].coordinate = new Coordinate(0, allEvents[i].coordinate.y);
    }
    printWindowedJitter();
}

document.getElementById('button1').onclick = amIBot;
document.getElementById('button3').onclick = setXtoZero;
document.getElementById('button2').onclick = printWindowedJitter;
/* For a live demo
document.getElementById('button1').onclick = showErr(amIBot);
document.getElementById('button2').onclick = showErr(printWindowedJitter);
document.getElementById('button3').onclick = showErr(setXtoZero);
*/

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
