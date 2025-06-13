/*
 * This is a demo project built using Tuurbo's pancake cat tutorial plus some additions. 
 * To run: turbo run -w pancake-ca 
*/

turbo::init! {
    struct GameState {
        frame: u32,
        last_munch_at: u32,
        cat_id: usize, 
        cat_x: f32,
        cat_y: f32,
        cat_r: f32,
        pancakes: Vec<struct Pancake {
            x: f32,
            y: f32,
            vel: f32,
            radius: f32,
        }>,
        score: u32,
        skin_change_button: UIButton, 
        mute_button: UIButton,
        mute_toggle: bool,

    } = Self {
        frame: 0,
        last_munch_at: 0,
        cat_id: 0, 
        cat_x: 128.0,
        cat_y: 112.0,
        cat_r: 8.0,
        pancakes: vec![],
        score: 0,
        skin_change_button: UIButton::new("new cat", (200, 10, 40, 10)),
        mute_button: UIButton::new("sound", (220, 130, 30, 10)),
        mute_toggle: false, 
    }
}

turbo::go!({
    let mut state = GameState::load();
    let gp = gamepad(0);
 
    // move the cat
    if gp.left.pressed() {
        state.cat_x -= 2.;
    }
    if gp.right.pressed() {
        state.cat_x += 2.;
    }

    // some pancake logic
    if rand() % 64 == 0 {
        let pancake = Pancake {
            x: (rand() % 256) as f32,
            y: 0.0,
            vel: (rand() % 3 + 1) as f32,
            radius: (rand() % 10 + 5) as f32,
        };
        state.pancakes.push(pancake);
    }

    let cat_center = (state.cat_x + state.cat_r, state.cat_y + state.cat_r);
 
    // collision logic
    state.pancakes.retain_mut(|p| {
        p.y += p.vel;
        let dx = cat_center.0 - (p.x + p.radius);
        let dy = cat_center.1 - (p.y + p.radius);
        let distance = (dx * dx + dy * dy).sqrt();
        let radii_sum = state.cat_r + p.radius;
        let radii_diff = (state.cat_r - p.radius).abs();
    
        if radii_diff <= distance && distance <= radii_sum {
            state.score += 1;
            state.last_munch_at = state.frame;

            // add munch sound only if sound enabled 
            if (!state.mute_toggle) {
                audio::play("munch");
            }

            false
        } else if p.y < 144. + (p.radius * 2.) {
            true
        } else {
            false
        }
    });

    // background logic
    clear(0x00ffffff);
    let frame = (state.frame as i32) / 2;
    for col in 0..9 {
        for row in 0..5 {
            let x = ((col * 32 + frame) % (272 + 16)) - 32;
            let y = ((row * 32 + frame) % (144 + 16)) - 24;
            sprite!("yum", x = x + 7, y = y + 7); 
        }
    }
    state.frame += 1;

    // https://www.youtube.com/watch?v=9WyMSCVG6ZE&list=PL4thXl4CNv5IjPSx9-_f-F60zZlsORw2z&index=19
    // new potential adds: 
    //   - change background to a different image - DONE 
    //   - click a button to change cat skins - DONE 
    //      - fix coloring on additional cats
    //   - add background music and a munch sound? - DONE 
    //   - add sound toggling - DONE 
    //   - add start/end page
    //   - add dynamite "die" -> drops you in start page (check bork runner mini game)

    let skins = vec![
        "munch_cat", 
        "munch_cat_white", 
        "munch_cat_black",
    ]; 

    // register mouse positions
    let m = mouse(0);
    let [mx, my] = m.position; 

    // click the "new cat" button to change the color of the cat
    if let Some(b) = state.skin_change_button.hover(state.skin_change_button.hitbox, mx, my) {
        if m.left.just_pressed() {
            let mut id = state.cat_id + 1; 
            // loop back to beginning of vector 
            if id >= skins.len() { id = 0 }
    
            // set cat id
            state.cat_id = id; 
        } 
    }

    // button for changing cat (top right)
    state.skin_change_button.draw(); 

    let mut current_sprite = skins[state.cat_id]; 

    // draw cat
    sprite!(current_sprite, x = state.cat_x - state.cat_r, y = state.cat_y - 16.0);

    // sound toggle for background
    if let Some(b) = state.mute_button.hover(state.mute_button.hitbox, mx, my) {
        if m.left.just_pressed() {
            state.mute_toggle = !state.mute_toggle; // flip on and off mute toggle
        }
    }

    // play background music
    if !state.mute_toggle {
        // loop background music 
        if !audio::is_playing("background") { audio::play("background"); }
    } else {
        audio::pause("background"); 
    }

    // draw sound button
    state.mute_button.draw(); 

    // draw pancakes
    for p in &state.pancakes {
        circ!(x = p.x, y = p.y + 1.0, d = p.radius + 2., color = 0x000000aa);
        circ!(x = p.x, y = p.y, d = p.radius + 1., color = 0xf4d29cff);
        circ!(x = p.x, y = p.y, d = p.radius, color = 0xdba463ff);
    }
    
    // munch textbox
    if state.frame >= 64 && state.frame.saturating_sub(state.last_munch_at) <= 60 {
        rect!(w = 30, h = 10, x = state.cat_x + 32.0, y = state.cat_y);
        circ!(d = 10, x = state.cat_x + 28.0, y = state.cat_y);
        rect!(w = 10, h = 5, x = state.cat_x + 28.0, y = state.cat_y + 5.0);
        circ!(d = 10, x = state.cat_x + 56.0, y = state.cat_y);
        text!("MUNCH!", x = state.cat_x + 33.0, y = state.cat_y + 3.0, font = "small", color = 0x000000ff); 
    }

    // score label (top left)
    text!("Score: {}", state.score; x = 10, y = 10, font = "large", color = 0xffffffff);
 
    state.save();
});

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct UIButton {
    pub hitbox: (i32, i32, i32, i32), // x, y, width, height 
    pub text: String, 
    pub hovered: bool, 
}

impl UIButton {
    pub fn new(text: &str, hitbox: (i32, i32, i32, i32)) -> Self {
        Self { 
            hitbox, 
            text: text.to_string(), 
            hovered: false, 
        }
    }

    pub fn draw(&self) {
        let (c1, c2): (u32, u32) = match self.hovered { 
            true => (0x003366FF, 0xffffffff),
            false => (0xffffffff, 0x003366FF),
        };

        rect!(x = self.hitbox.0, y = self.hitbox.1, w = self.hitbox.2, h = self.hitbox.3, color = c1);
        text!(&self.text, x = self.hitbox.0 + 3, y = self.hitbox.1, color = c2); 
    }
}

pub trait Clickable {
    // check if mouse hover over box
    fn hover(&mut self, hitbox: (i32, i32, i32, i32), mx: i32, my: i32) -> Option<&mut Self> {
        
        if mx >= hitbox.0 && mx <= hitbox.0 + hitbox.2 && my >= hitbox.1 && my <= hitbox.1 + hitbox.3 {
            // mouse is over box
            self.hover_toggle(true); 
            return Some(self)
        } else { 
            // mouse is not over box
            self.hover_toggle(false); 
            return None
        }
    }

    // always overridden 
    fn hover_toggle(&mut self, state: bool) {}
}

impl Clickable for UIButton {
    fn hover_toggle(&mut self, state: bool) {
        self.hovered = state; 
    }
}

