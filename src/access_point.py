import network
import rp2

class AccessPoint:
    wlan = network.WLAN(network.AP_IF)

    def setup(self) -> None:
        rp2.country("NL")

        self.wlan.config(ssid="fauxne", key="fake phone")
        self.wlan.active(True)
        # self.wlan.ifconfig(('192.168.0.1', '255.255.255.0', '192.168.0.1', '8.8.8.8'))

        status = self.wlan.ifconfig()
        print('ip = ' + status[0])
        print(status)