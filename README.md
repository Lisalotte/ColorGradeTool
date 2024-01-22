# Introduction 
ColorGradeTool is a simple GUI tool which can adjust the color values of multiple compositor layers in Unreal Engine.

The tool has a 'simple' side, which only shows buttons to select a saved configuration (and preset).
It also has a 'configuration' side, where you can adjust the color values using 3 sets of sliders.
You can then save this preset, and assign it to one of 10 buttons. These buttons will then be visible on the 'simple' side.

You can also save a configuration, which includes project/level name, IP address, object path, as well as the preset.

The object path points to a Blueprint object, which should be placed in your main level in Unreal Engine.

# Getting Started
1.	Run the program by using the .exe (in the folder target/release)
2.	See the config folder for the default.json config file
3.	Download the UE blueprints (ColorGrading.uasset and MI_ColorGrading.uasset)
    Place the ColorGrading asset in your main level.
    Use the MI_ColorGrading as your compositor color grading material.