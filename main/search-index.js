var searchIndex = JSON.parse('{\
"gooey_browser":{"doc":"","t":[3,11,11,11,8,10,8,10,11,11,11,11,11,11,11,11,11,11,11],"n":["WebSys","new","register_transmogrifier","install_in_id","WebSysTransmogrifier","transmogrify","AnyWidgetWebSysTransmogrifier","transmogrify","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into"],"q":["gooey_browser","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","",""],"i":[0,1,1,1,0,2,0,3,1,1,1,1,1,1,1,1,1,1,1],"f":[null,[[["gooey",3]]],[[["sync",8],["send",8],["websystransmogrifier",8]]],[[["str",15]]],null,[[["node",3]],[["element",3],["option",4]]],null,[[["node",3],["anywidget",8]],[["element",3],["option",4]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["outofbounds",3],["result",4]]]],"p":[[3,"WebSys"],[8,"WebSysTransmogrifier"],[8,"AnyWidgetWebSysTransmogrifier"]]},\
"gooey_core":{"doc":"Core traits and types used to create Graphical User …","t":[0,8,10,10,10,10,10,10,10,3,12,12,3,12,12,3,12,12,12,12,11,11,8,16,3,12,8,16,8,16,16,10,8,10,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["renderer","Renderer","size","scale","render_text","measure_text","stroke_rect","fill_rect","stroke_line","TextOptions","font_family","text_size","StrokeOptions","color","line_width","TextMetrics","width","ascent","descent","line_gap","height","line_height","Frontend","AnyWidgetTransmogrifier","Gooey","transmogrifiers","Widget","TransmogrifierEvent","Transmogrifier","Widget","Context","content_size","AnyWidget","as_any","widget_type_id","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","default","div","mul","new","root_widget"],"q":["gooey_core","gooey_core::renderer","","","","","","","","","","","","","","","","","","","","","gooey_core","","","","","","","","","","","","","","","","","","","","","","","","gooey_core::renderer","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","gooey_core",""],"d":["Types used for drawing.","Implements drawing APIs.","The size of the area being drawn.","The scale when converting between [<code>Points</code>] and [<code>Pixels</code>].","Renders <code>text</code> at <code>baseline_origin</code> with <code>options</code>.","Measures <code>text</code> using <code>options</code>.","Strokes the outline of <code>rect</code> using <code>options</code>.","Fills <code>rect</code> using <code>color</code>.","Draws a line between <code>point_a</code> and <code>point_b</code> using <code>options</code>.","Text rendering and measurement options.","The font family to use.","The text size, in [<code>Points</code>].","Shape outline drawing options.","The color to stroke.","The width of each line segment.","A measurement of text.","The width of the text.","The height above the baseline.","The height below the baseline. Typically a negative …","The spacing expected between lines of text.","The height of the rendered text. This is computed by …","The height of a line of text. This is computed by …","A frontend is an implementation of widgets and layouts.","The generic-free type of the frontend-specific …","A graphical user interface.","The available widget transmogrifiers.","A graphical user interface element.","The type of the event that any [<code>Transmogrifier</code>] for this …","Transforms a Widget into whatever is needed for [<code>Frontend</code>]…","The type of the widget being transmogrified.","The frontend-specific context type provided to aide in …","Calculate the content-size needed for this <code>widget</code>, trying …","A Widget without any associated types. Useful for …","Returns the widget as the [<code>Any</code>] type.","Returns the [<code>TypeId</code>] of the widget.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Creates a user interface using <code>root</code>.","Returns the root widget."],"i":[0,0,1,1,1,1,1,1,1,0,2,2,0,3,3,0,4,4,4,4,4,4,0,5,0,6,0,7,0,8,8,8,0,9,9,6,6,6,6,6,6,6,6,6,6,6,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,4,4,4,2,4,4,6,6],"f":[null,null,[[],[["points",3],["size2d",3],["f32",15]]],[[],[["pixels",3],["scale",3],["points",3],["f32",15]]],[[["textoptions",3],["f32",15],["points",3],["point2d",3],["str",15]]],[[["textoptions",3],["str",15]],[["textmetrics",3],["points",3]]],[[["rect",3],["strokeoptions",3]]],[[["rect",3],["srgba",6]]],[[["f32",15],["point2d",3],["points",3],["strokeoptions",3]]],null,null,null,null,null,null,null,null,null,null,null,[[],[["length",3],["f32",15]]],[[],[["length",3],["f32",15]]],null,null,null,null,null,null,null,null,null,[[["option",4],["size2d",3],["points",3]],[["points",3],["size2d",3],["f32",15]]],null,[[],["any",8]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["result",4],["outofbounds",3]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["result",4],["outofbounds",3]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["result",4],["outofbounds",3]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["result",4],["outofbounds",3]]],[[]],[[["scale",3],["f32",15]]],[[["scale",3],["f32",15]]],[[["sync",8],["widget",8],["send",8]]],[[],["anywidget",8]]],"p":[[8,"Renderer"],[3,"TextOptions"],[3,"StrokeOptions"],[3,"TextMetrics"],[8,"Frontend"],[3,"Gooey"],[8,"Widget"],[8,"Transmogrifier"],[8,"AnyWidget"]]},\
"gooey_kludgine":{"doc":"","t":[3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Kludgine","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","downcast","upcast","init","deref","deref_mut","drop","from","size","scale","render_text","measure_text","stroke_rect","fill_rect","stroke_line"],"q":["gooey_kludgine","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[null,[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["result",4],["outofbounds",3]]],[[]],[[],["option",4]],[[],["usize",15]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["target",3]]],[[],[["points",3],["size2d",3],["f32",15]]],[[],[["points",3],["pixels",3],["f32",15],["scale",3]]],[[["points",3],["point2d",3],["f32",15],["textoptions",3],["str",15]]],[[["textoptions",3],["str",15]],[["textmetrics",3],["points",3]]],[[["strokeoptions",3],["rect",3]]],[[["srgba",6],["rect",3]]],[[["points",3],["strokeoptions",3],["f32",15],["point2d",3]]]],"p":[[3,"Kludgine"]]},\
"gooey_rasterizer":{"doc":"","t":[3,11,11,11,11,11,8,10,8,10,11,11,11,11,11,11,11,11,11,11,11],"n":["Rasterizer","new","render","register_transmogrifier","transmogrifier","root_transmogrifier","WidgetRasterizer","render","AnyWidgetRasterizer","render","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into"],"q":["gooey_rasterizer","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","",""],"i":[0,1,1,1,1,1,0,2,0,3,1,1,1,1,1,1,1,1,1,1,1],"f":[null,[[["gooey",3]]],[[]],[[["sync",8],["send",8],["widgetrasterizer",8]]],[[["typeid",3]],[["anywidgetrasterizer",8],["option",4]]],[[],[["anywidgetrasterizer",8],["option",4]]],null,[[["rect",3],["f32",15],["points",3]]],null,[[["rect",3],["f32",15],["anywidget",8],["points",3]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["result",4],["outofbounds",3]]]],"p":[[3,"Rasterizer"],[8,"WidgetRasterizer"],[8,"AnyWidgetRasterizer"]]},\
"gooey_widgets":{"doc":"","t":[0,3,12,12,4,13,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["button","Button","label","disabled","ButtonEvent","Clicked","ButtonTransmogrifier","from","into","to_owned","clone_into","borrow","borrow_mut","try_from","try_into","type_id","as_any","widget_type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","from","into","borrow","borrow_mut","try_from","try_into","type_id","adapt_into_using","convert_into","convert_unclamped_into","try_convert_into","clone","eq","ne"],"q":["gooey_widgets","gooey_widgets::button","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,0,2,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,3,3,3,1,1,1],"f":[null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["any",8]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["outofbounds",3],["result",4]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["outofbounds",3],["result",4]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[],[["outofbounds",3],["result",4]]],[[],["button",3]],[[["button",3]],["bool",15]],[[["button",3]],["bool",15]]],"p":[[3,"Button"],[4,"ButtonEvent"],[3,"ButtonTransmogrifier"]]},\
"xtask":{"doc":"","t":[4,13,5,5,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["Args","BuildBrowserExample","main","build_browser_example","from","into","borrow","borrow_mut","try_from","try_into","type_id","fmt","clap","from_clap","augment_clap","from_subcommand","is_subcommand"],"q":["xtask","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","",""],"i":[0,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[null,null,[[],[["error",3],["result",4]]],[[],[["error",3],["result",4]]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["formatter",3]],["result",6]],[[],["app",3]],[[["argmatches",3]]],[[["app",3]],["app",3]],[[],["option",4]],[[],["bool",15]]],"p":[[4,"Args"]]}\
}');
initSearch(searchIndex);