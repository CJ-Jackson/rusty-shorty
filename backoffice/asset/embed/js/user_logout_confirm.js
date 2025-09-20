function logoutConfirm() {
    // get class js-logout-confirm
    let elements = document.getElementsByClassName("js-logout-confirm");
    for (let element of elements) {
        element.onclick = function (e) {
            // username from data
            let username = this.getAttribute("data-username");
            return confirm("Are you sure you want to logout '" + username + "'?");
        }
    }
}

logoutConfirm();