document.addEventListener('DOMContentLoaded', () => {
    const connectionStatus = document.getElementById('connection-status');
    const load1 = document.getElementById('load-1');
    const load5 = document.getElementById('load-5');
    const load15 = document.getElementById('load-15');
    const serverTime = document.getElementById('server-time');

    function updateConnectionStatus(connected) {
        connectionStatus.textContent = connected ? 'Connected to Server' : 'Disconnected from Server';
        connectionStatus.className = connected ? 'connected' : 'disconnected';
    }

    function formatTime(timestamp) {
        return new Date(parseInt(timestamp)).toLocaleString();
    }

    const eventSource = new EventSource('/stats');

    eventSource.onopen = () => {
        updateConnectionStatus(true);
        console.log('Connected to server');
    };

    eventSource.onerror = () => {
        updateConnectionStatus(false);
        console.log('Connection error');
    };

    eventSource.addEventListener('uptime', (event) => {
        const data = JSON.parse(event.data);
        load1.textContent = data.one_min.toFixed(2);
        load5.textContent = data.five_min.toFixed(2);
        load15.textContent = data.fifteen_min.toFixed(2);
    });

    eventSource.addEventListener('time', (event) => {
        serverTime.textContent = formatTime(event.data);
    });
});