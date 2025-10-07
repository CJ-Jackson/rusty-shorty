import htmx from 'htmx'

htmx.config.responseHandling = [
    {code: '204', swap: false},
    {code: '[23]..', swap: true},
    {code: '422', swap: true},
    {code: '[45]..', swap: false, error: true}
];

function formatToLocalTime() {
    let elements = document.getElementsByClassName("js-date-local");
    for (let element of elements) {
        let date = new Date(element.innerHTML);
        if (isNaN(date.getTime()) || date.toString() === "Invalid Date" || date.getTime() === 0) {
            return;
        }
        element.innerHTML = date.toLocaleString();
        element.classList.remove("js-date-local");
    }
}

formatToLocalTime();

async function clearNavActive() {
    let element = document.getElementById("tag-update");
    if (element !== null) {
        let elements = document.getElementsByClassName("nav-item");
        for (let element of elements) {
            element.classList.remove("nav-item-active");
        }
    }
}

function addNavActive() {
    let element = document.getElementById("tag-update");
    if (element !== null) {
        if (element.dataset.tag === undefined || element.dataset.tag === "") {
            return;
        }
        let tagElement = document.getElementById(element.dataset.tag);
        if (tagElement !== null) {
            tagElement.classList.add("nav-item-active");
        }
    }
}

htmx.onLoad(function () {
    formatToLocalTime();
    clearNavActive().then(function () {
        addNavActive();
    });
});