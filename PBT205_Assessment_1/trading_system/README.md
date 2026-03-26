# Prototype 2: Trading System

This project is a simple trading system built with Python, Docker, and RabbitMQ. It simulates trading for one stock, **XYZ Corp**, where traders submit BUY and SELL orders, the exchange matches compatible orders, and the latest trade is shown in a GUI.

## Files

- `sendOrder.py` — sends BUY or SELL orders
- `exchange.py` — receives orders, manages the order book, and matches trades
- `trade_listener.py` — shows completed trades in the terminal
- `trade_gui.py` — shows the latest trade in a graphical interface

## Requirements

- Python 3.9+
- Docker Desktop
- `pika`

# Start RabbitMQ
docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management

# Optional management page: http://localhost:15672

Run the Project
1. Start the exchange
python exchange.py localhost:5672
2. Start the trade listener
python trade_listener.py
3. Start the GUI
python trade_gui.py localhost:5672
4. Send sample orders
python sendOrder.py bob localhost:5672 SELL 100 11.8
python sendOrder.py alice localhost:5672 BUY 100 12.5
