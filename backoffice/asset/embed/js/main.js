import htmx from 'htmx'

htmx.config.responseHandling = [
    { code: '204', swap: false },
    { code: '[23]..', swap: true },
    { code: '422', swap: true },
    { code: '[45]..', swap: false, error: true }
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

function messageConfirm() {
    let elements = document.getElementsByClassName("js-message-confirm");
    for (let element of elements) {
        element.onclick = function (e) {
            let msg = this.getAttribute("data-msg");
            return confirm(msg);
        }
        element.classList.remove("js-message-confirm");
    }
}

htmx.onLoad(function () {
    formatToLocalTime();
    messageConfirm();
});