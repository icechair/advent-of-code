#!/usr/bin/env lua

function Read_all(file)
  local f = io.open(file, 'rb')
  if f then
    local content = f:read '*all'
    f:close()
    return content
  end
  print('error: f not found ' .. file)
  return ''
end

local year = arg[1]
if not year then
  year = '2015'
end
local day = arg[2]
if not day then
  day = arg[2]
end

local url = 'https://adventofcode.com/' .. year .. '/day/' .. day .. '/input'
local cookie = Read_all('.cookie'):gsub('%s+', '')
local command = 'curl ' .. url .. " -H 'cookie: " .. cookie .. "' -o input.txt"
print(command)

local h_cmd = io.popen(command)
if h_cmd then
  local result = h_cmd:read '*all'
  print(result)
  h_cmd:close()
end
