function stackClearConfirm() {
    // get class js-logout-confirm
    let elements = document.getElementsByClassName("js-clear-confirm");
    for (let element of elements) {
        element.onclick = function (e) {
            return confirm("Are you sure you want to clear message older than 30 days?");
        }
    }
}

stackClearConfirm();