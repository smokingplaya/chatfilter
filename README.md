# ChatFilter

# Using
Create ``chatfilter.lua`` file in ``GarrysModDS/garrysmod/lua/autorun/server.``
```lua
require("chatfilter")

chatfilter:SetFilter {
  "fuck",
  "mf",
  "wtf",
  "example"
}

hook.Add("PlayerSay", "smokingplaya.chatfilter", function(pl, msg)
  if pl:IsSuperAdmin() then
    return
  end

  if chatfilter:Check(msg) then
    print("Player " .. pl:Nick() .. " wrote a banned word")

    return ""
  end
end)
```
