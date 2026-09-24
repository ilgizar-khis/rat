local r = rat.new({ draw = true })

rat.move_dir({ id = r, dir = "right", times = 30 })
for i = 1, 10 do
	rat.move_to({ id = r, pos = { 0, i * 3 }, times = 1 })
	rat.move_dir({ id = r, dir = "right", times = 30 })
end

rat.move_to({ id = r, pos = { 0, 0 }, times = 1 })
rat.move_dir({ id = r, dir = "down", times = 30 })

for i = 1, 10 do
	rat.move_to({ id = r, pos = { i * 3, 0 }, times = 1 })
	rat.move_dir({ id = r, dir = "down", times = 30 })
end
