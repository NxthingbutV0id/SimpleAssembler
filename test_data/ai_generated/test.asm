// Define constants for controller input bits (assuming these mappings based on examples)
define UP_BIT 8    // Corresponds to 'W'
define RIGHT_BIT 1  // Corresponds to 'A' (now right)
define DOWN_BIT 2  // Corresponds to 'S'
define LEFT_BIT 4 // Corresponds to 'D' (now left)

// Initialize pixel position (start at some default location)
ldi r1 16      // r1 will store the X coordinate (start at center)
ldi r2 16      // r2 will store the Y coordinate (start at center)

.main_loop
    // Read controller input
    ldi r15 controller_input
    lod r15 r3     // r3 now holds the controller input

    // Check for 'W' (Up)
    ldi r14 UP_BIT
    and r3 r14 r14
    brh ne .check_d // 'W' not pressed, check 'D' (which is now left)
    // 'W' is pressed, move pixel up
    dec r2
    brh nz .keep_y_bounds_up // Ensure Y is not negative (greater than -1)
    ldi r2 0             // If it is, set Y to 0
    .keep_y_bounds_up

.check_d
    // Check for 'D' (Left - now inverted)
    ldi r14 LEFT_BIT
    and r3 r14 r14
    brh ne .check_s // 'D' not pressed, check 'S'
    // 'D' is pressed, move pixel left
    dec r1
    brh nz .keep_x_bounds_left // Ensure X is not negative (greater than -1)
    ldi r1 0               // If it is, set X to 0
    .keep_x_bounds_left

.check_s
    // Check for 'S' (Down)
    ldi r14 DOWN_BIT
    and r3 r14 r14
    brh ne .check_a // 'S' not pressed, check 'A' (which is now right)
    // 'S' is pressed, move pixel down
    inc r2
    ldi r14 31         // Maximum Y coordinate is 31
    cmp r2 r14
    brh nc .y_is_within_bounds_down
    mov r14 r2
.y_is_within_bounds_down

.check_a
    // Check for 'A' (Right - now inverted)
    ldi r14 RIGHT_BIT
    and r3 r14 r14
    brh ne .update_screen // 'A' not pressed, proceed to update screen
    // 'A' is pressed, move pixel right
    inc r1
    ldi r14 31         // Maximum X coordinate is 31
    cmp r1 r14
    brh nc .x_is_within_bounds_right
    mov r14 r1
.x_is_within_bounds_right

.update_screen
    // Clear the pixel at the previous position
    ldi r15 pixel_x
    str r15 r1
    ldi r15 pixel_y
    str r15 r2
    ldi r15 clear_pixel
    str r15 r0

    // Draw the pixel at the new position
    ldi r15 pixel_x
    str r15 r1
    ldi r15 pixel_y
    str r15 r2
    ldi r15 draw_pixel
    str r15 r0

    // Update the screen buffer to display the changes
    ldi r15 buffer_screen
    str r15 r0

    // Loop back to check for more input
    jmp .main_loop

// --- END OF PROGRAM ---