import htmx from 'htmx'

async function stub() {
}

function formatToLocalTime() {
    stub().then(function () {
        let elements = document.getElementsByClassName("js-date-local");
        for (let element of elements) {
            let date = new Date(element.innerHTML);
            if (isNaN(date.getTime()) || date.toString() === "Invalid Date" || date.getTime() === 0) {
                return;
            }
            element.innerHTML = date.toLocaleString();
        }
    }).then(function () {
        let elements = document.getElementsByClassName("js-date-local");
        for (let element of elements) {
            element.classList.remove("js-date-local");
        }
    })
}

async function clearNavActive() {
    let elements = document.getElementsByClassName("nav-item");
    for (let element of elements) {
        element.classList.remove("nav-item-active");
    }
}

function addNavActive() {
    let tagUpdateElement = document.getElementById("tag-update");
    if (tagUpdateElement !== null) {
        clearNavActive().then(function () {
            if (tagUpdateElement.dataset.tag === undefined || tagUpdateElement.dataset.tag === "") {
                return;
            }
            let tagElement = document.getElementById(tagUpdateElement.dataset.tag);
            if (tagElement !== null) {
                tagElement.classList.add("nav-item-active");
            }
        }).then(function () {
            tagUpdateElement.remove();
        });
    }
}

htmx.onLoad(function () {
    formatToLocalTime();
    addNavActive();
});