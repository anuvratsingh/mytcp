### How to create a virtual network adaptor
1. Create a new tun virtual device: `sudo ip tuntap add mode tun name <adaptor-name> user <username>`
2. Assign IP address to it: `sudo ip addr add 192.168.42.100/24 dev <adaptor-name>`
3. Turn on/off the adaptor: `sudo ip link set up/down dev <adaptor-name>`

- Note: Adaptor name used in `tun0`