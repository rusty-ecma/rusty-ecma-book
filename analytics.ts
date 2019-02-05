import { sendExiting, sendInfo, initialResponseHandler,setup_click_watcher, notifyUser, notificationNeeded } from '../analytics/analytics';

window.addEventListener('DOMContentLoaded', () => {
    setup_click_watcher();
    if (notificationNeeded()) {
        notifyUser();
    }

});
window.addEventListener('beforeunload', () => {
    sendExiting();
});
window.addEventListener('load', () => {
    sendInfo("https://wiredforge.com/analytics/landing").then(initialResponseHandler).catch(e => {});
});