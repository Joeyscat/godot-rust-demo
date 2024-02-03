extends Player

var client = null

# Called when the node enters the scene tree for the first time.
func _ready():
	client = Client.new("192.168.50.5:8080")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func _input(event):
	pass
