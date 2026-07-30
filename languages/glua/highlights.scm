; Keywords
[
  "do"
  "else"
  "elseif"
  "end"
  "for"
  "function"
  "goto"
  "if"
  "in"
  "local"
  "global"
  "repeat"
  "return"
  "then"
  "until"
  "while"
  (break_statement)
] @keyword

; Operators
[
  "and"
  "not"
  "or"
] @keyword.operator

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  "#"
  "=="
  "~="
  "<="
  ">="
  "<"
  ">"
  "="
  "&"
  "~"
  "|"
  "<<"
  ">>"
  "//"
  ".."
] @operator

; Punctuations
[
  ";"
  ":"
  ","
  "."
] @punctuation.delimiter

; Brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Variables
(identifier) @variable

((identifier) @variable.special
  (#eq? @variable.special "self"))

(variable_list
  attribute: (attribute
    ([
      "<"
      ">"
    ] @punctuation.bracket
      (identifier) @attribute)))

; Constants
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]*$"))

(vararg_expression) @constant

(nil) @constant.builtin

[
  (false)
  (true)
] @boolean

; Tables
(field
  name: (identifier) @property)

(dot_index_expression
  field: (identifier) @property)

(table_constructor
  [
    "{"
    "}"
  ] @constructor)

; Functions
(parameters
  (identifier) @parameter)

(function_call
  name: [
    (identifier) @function
    (dot_index_expression
      field: (identifier) @function)
  ])

(function_declaration
  name: [
    (identifier) @function.definition
    (dot_index_expression
      field: (identifier) @function.definition)
  ])

(method_index_expression
  method: (identifier) @function.method)

; Lua 5.1 built-in functions
(function_call
  (identifier) @function.builtin
  (#any-of? @function.builtin
    "assert" "collectgarbage" "dofile" "error" "getfenv" "getmetatable" "ipairs" "load" "loadfile"
    "loadstring" "module" "next" "pairs" "pcall" "print" "rawequal" "rawget" "rawset" "require"
    "select" "setfenv" "setmetatable" "tonumber" "tostring" "type" "unpack" "xpcall"))

; GMod/GLua specific built-in functions
(function_call
  (identifier) @function.builtin
  (#any-of? @function.builtin
    ; Filesystem
    "file.Exists" "file.IsDir" "file.Read" "file.Write" "file.Delete" "file.Rename"
    "file.Find" "file.Size" "file.Time" "file.TMod" "file.Open" "file.Append"
    "file.CreateDir" "file.Decompress" "file.Walk" "file.IsBinary" "file.IsEmpty"
    "file.ReadStream" "file.WriteStream"
    ; HTTP
    "HTTP" "http.Fetch" "http.Post" "http.Get" "http.Put"
    ; Utilities
    "include" "AddCSLuaFile" "IncludeCS" "CompileFile" "CompileString" "RunString"
    "RunStringEx" "Msg" "MsgN" "MsgC" "MsgAll" "ErrorNoHalt" "ErrorNoHaltWithStack"
    "PrintTable" "DeveloperMsg" "Print"
    ; Entity
    "Entity" "ents.FindByClass" "ents.FindInSphere" "ents.FindInBox"
    "ents.FindByModel" "ents.FindByName" "player.GetByID" "player.GetAll"
    ; Rendering
    "surface.CreateFont" "surface.SetFont" "surface.GetTextSize" "surface.SetTextPos"
    "surface.SetDrawColor" "surface.DrawRect" "surface.DrawRect" "surface.DrawLine"
    "surface.DrawCircle" "surface.DrawPoly" "surface.SetMaterial" "surface.SetTexture"
    "surface.DrawTexturedRect" "draw.DrawText" "draw.RoundedBox" "draw.NoTexture"
    "cam.Start2D" "cam.End2D" "cam.Start3D" "cam.End3D"
    ; Math
    "math.Clamp" "math.Approach" "math.Round" "math.Rand" "math.randomseed"
    "math.Ceil" "math.Floor" "math.Remainder" "math.fmod" "math.Sign"
    "math.Lerp" "math.LerpAngle" "math.ApproachAngle" "math.NormalizeAngle"
    "math.AngleDifference" "math.SharedAngle" "math.ease"
    ; Color
    "Color" "ColorAlpha" "HSVToColor" "ColorToHSV"
    ; String
    "string.find" "string.gmatch" "string.gsub" "string.match" "string.reverse"
    "string.rep" "string.sub" "string.format" "string.byte" "string.char"
    "string.lower" "string.upper" "string.Trim" "string.TrimLeft" "string.TrimRight"
    "string.StartWith" "string.EndsWith" "string.Explode" "string.Implode"
    "string.GetExtensionFromFilename" "string.GetFileFromFilename" "string.GetPathFromFilename"
    "string.NiceSize" "string.NiceTime" "string.NiceFloat" "string.Comma"
    ; Table
    "table.insert" "table.remove" "table.sort" "table.concat" "table.Copy"
    "table.Empty" "table.Find" "table.filter" "table.ForEach" "table.ForEachI"
    "table.GetFirstKey" "table.GetLastKey" "table.GetKeys" "table.GetN"
    "table.HasValue" "table.key" "table.MaxN" "table.Merge" "table.Reverse"
    "table.Shuffle" "table.Random" "table.ToString"
    ; Timer
    "timer.Create" "timer.Simple" "timer.Adjust" "timer.Remove" "timer.Exists"
    "timer.Stop" "timer.Start" "timer.Toggle" "timer.Pause" "timer.UnPause"
    "timer.Update" "timer.GetTable"
    ; Hook
    "hook.Add" "hook.Remove" "hook.Call" "hook.Run"
    ; Gamemode
    "GM" "GAMEMODE" "DerivedGamemode"
    ; Data
    "datastore.GetTable" "datastore.SetTable" "datastore.Get" "datastore.Set"
    "datastore.Exists" "datastore.Remove" "datastore.Save" "datastore.Load"
    ; JSON
    "util.JSONToTable" "util.TableToJSON" "util.Decompress" "util.Compress"
    "util.CRC" "util.MD5" "util.SHA1" "util.CalculateMD5" "util.CalculateSHA1"
    ; Particle
    "particle.Create" "particle.Emit" "PrecacheParticleSystem"
    ; Sound
    "surface.PlaySound" "sound.PlayFile" "sound.PlayURL" "LocalPlayer():EmitSound"
    ; Network
    "net.Start" "net.Send" "net.Broadcast" "net.SendToServer" "net.Receive"
    "net.WriteUInt" "net.WriteInt" "net.WriteBool" "net.WriteFloat" "net.WriteString"
    "net.WriteBit" "net.WriteEntity" "net.WriteVector" "net.WriteAngle" "net.WriteColor"
    "net.WriteTable" "net.WriteData" "net.WriteUInt" "net.WriteUInt64"
    "net.ReadUInt" "net.ReadInt" "net.ReadBool" "net.ReadFloat" "net.ReadString"
    "net.ReadBit" "net.ReadEntity" "net.ReadVector" "net.ReadAngle" "net.ReadColor"
    "net.ReadTable" "net.ReadData" "net.ReadUInt" "net.ReadUInt64"
    "net.Start" "net.Send" "net.Broadcast" "net.SendToServer"
    ; Usermessage (legacy)
    "umsg.Start" "umsg.End" "umsg.Short" "umsg.Long" "umsg.Float" "umsg.Char"
    "umsg.Bool" "umsg.String" "umsg.Entity" "umsg.Vector" "umsg.Angle" "umsg.UserMessage"
    ; File
    "file.Exists" "file.IsDir" "file.Read" "file.Write" "file.Delete" "file.Rename"
    "file.Find" "file.Size" "file.Time" "file.TMod" "file.Open" "file.Append"
    ; Panel/GUI
    "vgui.Create" "vgui.CreateX" "DermaMenu" "Derma_StringRequest" "Derma_Message"
    "Derma_Query" "Derma_Anim" "Derma_Highlight" "DFrame" "DPanel" "DButton"
    ; Render utilities
    "render.Clear" "render.ClearDepth" "render.SetScissorRect" "render.SetBlend"
    "render.SetColorModulation" "render.SetColorCorrection" "render.GetToneMappingScaleLinear"
    "render.SetAmbientLight" "render.GetAmbientLightColor" "render.GetColorDepth"
    "render.GetSurfaceColor" "render.GetSpecularPower" "render.GetFlashlightPos"
    "render.FlashlightIsOn" "render.UpdateScreenEffectTexture" "render.UpdateRefractTexture"
    ; Util misc
    "util.AddNetworkString" "util.AddClientString" "util.PrecacheModel" "util.PrecacheSound"
    "util.PrecacheSentence" "util.PrecacheScriptEffect" "util.PrecacheSurface"
    "util.Effect" "util.ScreenShake" "util.BlastDamage" "util.BlastDamageInfo"
    "util.DamageInfo" "util.GetPixelMap" "util.GetPixelColor"
    ; Bit
    "bit.badd" "bit.bsub" "bit.bmul" "bit.bdiv" "bit.bmod" "bit.bpow"
    "bit.bnot" "bit.band" "bit.bor" "bit.bxor" "bit.lshift" "bit.rshift"
    "bit.arshift" "bit.bclear" "bit.bextract" "bit.breplace" "bit.brotl" "bit.bror"
    ; Debug
    "debug.getinfo" "debug.getlocal" "debug.setlocal" "debug.getupvalue"
    "debug.setupvalue" "debug.getregistry" "debug.getmetatable" "debug.setmetatable"
    "debug.getfenv" "debug.setfenv" "debug traceback" "debug.getuservalue"
    "debug.setuservalue" "debug.upvalueid" "debug.upvaluejoin"))

; Others
(comment) @comment

(hash_bang_line) @preproc

(number) @number

(string) @string

(escape_sequence) @string.escape
