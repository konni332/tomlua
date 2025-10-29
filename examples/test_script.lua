if toggle == nil then
	error("global variable 'toggle' is not defined")
end

toggle = not toggle

for i, script in ipairs(scripts) do
	print(script:name())
end
