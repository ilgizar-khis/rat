local draw1 = true
local draw2 = false

local r1 = rat.new({ draw = draw1, pos = { 20, 20 } })
local r2 = rat.new({ draw = draw2, pos = { 19, 20 }, color = "#ff6767" })

local r1_dirs = { "right", "down", "left", "up" }
local r2_dirs = { "left", "up", "right", "down" }

for i = 1, 20 do
	rat.move_dir({ id = r1, dir = r1_dirs[i % 4 + 1], times = i })
	rat.move_dir({ id = r2, dir = r2_dirs[i % 4 + 1], times = i })
	draw1 = not draw1
	draw2 = not draw2
	rat.set_draw({ id = r1, draw = draw1 })
	rat.set_draw({ id = r2, draw = draw2 })
end
