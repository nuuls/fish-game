/*
// Add event listeners
// MOUSEDOWN
{
    let drag = drag.clone();
    let mousedown_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
        *drag.borrow_mut() = true;
    }) as Box<dyn FnMut(MouseEvent)>);
    event_target
        .add_event_listener_with_callback("mousedown", mousedown_cb.as_ref().unchecked_ref())
        .unwrap();
    mousedown_cb.forget();
}
// MOUSEUP and MOUSEOUT
{
    let drag = drag.clone();
    let mouseup_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
        *drag.borrow_mut() = false;
    }) as Box<dyn FnMut(MouseEvent)>);
    event_target
        .add_event_listener_with_callback("mouseup", mouseup_cb.as_ref().unchecked_ref())
        .unwrap();
    event_target
        .add_event_listener_with_callback("mouseout", mouseup_cb.as_ref().unchecked_ref())
        .unwrap();
    mouseup_cb.forget();
}
// MOUSEMOVE
{
    let theta = theta.clone();
    let phi = phi.clone();
    let canvas_width = canvas_width.clone();
    let canvas_height = canvas_height.clone();
    let dX = dX.clone();
    let dY = dY.clone();
    let drag = drag.clone();
    let mousemove_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
        if *drag.borrow() {
            let cw = *canvas_width.borrow();
            let ch = *canvas_height.borrow();
            *dX.borrow_mut() = (event.movement_x() as f32) * 2.0 * PI / cw;
            *dY.borrow_mut() = (event.movement_y() as f32) * 2.0 * PI / ch;
            *theta.borrow_mut() += *dX.borrow();
            *phi.borrow_mut() += *dY.borrow();
        }
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);
    event_target
        .add_event_listener_with_callback("mousemove", mousemove_cb.as_ref().unchecked_ref())
        .unwrap();
    mousemove_cb.forget();
}
*/
