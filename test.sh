max=10
for i in `seq 2 $max`
do
    python3 tcp_client.py &
done
