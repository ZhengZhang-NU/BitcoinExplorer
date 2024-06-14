import React, { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import 'chartjs-adapter-date-fns';
import {
    Chart as ChartJS,
    TimeScale,
    LinearScale,
    LineElement,
    PointElement,
    Tooltip,
    Legend,
    Title,
    ChartOptions,
} from 'chart.js';

ChartJS.register(TimeScale, LinearScale, LineElement, PointElement, Tooltip, Legend, Title);

const RealTimeChart: React.FC = () => {
    const [chartData, setChartData] = useState<{ x: Date; y: number }[]>([]);

    useEffect(() => {
        const ws = new WebSocket('wss://stream.binance.com:9443/ws/btcusdt@trade');

        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            const newPoint = { x: new Date(data.E), y: parseFloat(data.p) };
            setChartData((prevData) => [...prevData, newPoint]);
        };

        ws.onerror = (event) => {
            console.error("WebSocket error observed:", event);
        };

        ws.onclose = (event) => {
            console.log("WebSocket is closed now:", event);
        };

        return () => {
            ws.close();
        };
    }, []);

    const data = {
        datasets: [
            {
                label: 'BTC Price',
                data: chartData,
                fill: false,
                backgroundColor: 'rgba(75,192,192,1)',
                borderColor: 'rgba(75,192,192,1)',
            },
        ],
    };

    const options: ChartOptions<'line'> = {
        responsive: true,
        plugins: {
            legend: {
                display: true,
                position: 'top',
            },
            title: {
                display: true,
                text: 'Real-Time BTC Price',
            },
        },
        scales: {
            x: {
                type: 'time',
                time: {
                    unit: 'minute',
                    tooltipFormat: 'HH:mm:ss',
                    displayFormats: {
                        minute: 'HH:mm:ss',
                    },
                },
                title: {
                    display: true,
                    text: 'Time',
                },
            },
            y: {
                title: {
                    display: true,
                    text: 'Price (USD)',
                },
                ticks: {
                    callback: function (value: number | string) {
                        return `$${value}`;
                    },
                },
            },
        },
    };

    return (
        <div>
            <h2>Real-Time BTC Price</h2>
            <Line data={data} options={options} />
        </div>
    );
};

export default RealTimeChart;
