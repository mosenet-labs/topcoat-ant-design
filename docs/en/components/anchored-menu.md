# Anchored action menu

`anchored_menu` uses the browser Popover API and CSS anchors so actions inside a scrollable table can open above the table. It supports outside click and Escape dismissal, and closes after an item is clicked.

Apply `anchored_menu_trigger_attributes(cx, id)` to the trigger button. The `id` must come from trusted code because it also names the CSS anchor.
