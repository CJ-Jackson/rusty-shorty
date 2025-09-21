function shortyDeleteConfirm() {
    // get class js-logout-confirm
    let elements = document.getElementsByClassName("js-delete-confirm");
    for (let element of elements) {
        element.onclick = function (e) {
            let del = this.getAttribute("data-delete");
            return confirm("Are you sure you want to delete '" + del + "'?");
        }
    }
}

shortyDeleteConfirm();