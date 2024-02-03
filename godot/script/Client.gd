
class_name Client

var inner = Client_.new()
var connected = false

func _init(addr):
	var status = inner.connect(addr, 3)
	if status == 0:
		print("connected")
		connected = true
	elif status == 1:
		print("ERROR connect error")
	elif status == 2:
		print("ERROR already connect")
	else:
		print("ERROR unknown status")

func send(msg):
	if !connected:
		return
	inner.send(msg)

func receive():
	if !connected:
		return
	return inner.receive()
