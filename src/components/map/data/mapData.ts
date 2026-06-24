// ── Map data coordinates (in local world space, centered at 0,0) ──

export interface ContourData {
  points: number[]  // flattened [x,y,x,y,...]
}

export interface RoadData {
  points: number[]  // flattened [x,y,x,y,...]
}

export interface LocationData {
  id: string
  name: string
  x: number
  y: number
  type: "town" | "dungeon" | "wilderness" | "landmark"
}

// ── Contour lines (topographic rings) ─────────────────────
// Each contour is an array of x,y pairs forming a closed or open curve
export const contours: ContourData[] = [
  // Outermost ring
  { points: [
    -320, -180, -280, -240, -180, -280, -60, -300, 60, -290,
    180, -260, 280, -200, 320, -120, 330, -20, 310, 80,
    260, 180, 180, 250, 80, 290, -40, 300, -160, 280,
    -270, 220, -330, 140, -340, 40,
  ] },
  { points: [
    -260, -140, -220, -190, -130, -220, -40, -240, 50, -230,
    140, -200, 220, -150, 250, -80, 260, 0, 240, 70,
    200, 140, 130, 190, 50, 220, -30, 230, -110, 210,
    -190, 170, -240, 110, -260, 40,
  ] },
  { points: [
    -200, -100, -160, -140, -80, -160, 0, -170, 60, -160,
    120, -130, 170, -90, 190, -30, 195, 30, 180, 80,
    145, 130, 90, 160, 20, 175, -50, 165, -110, 140,
    -160, 95, -185, 40, -190, -10,
  ] },
  { points: [
    -140, -60, -100, -90, -40, -105, 20, -100, 70, -80,
    110, -50, 125, 0, 120, 50, 95, 85, 50, 105,
    -10, 110, -60, 100, -100, 75, -120, 35, -120, -5,
  ] },
  { points: [
    -75, -25, -45, -45, -10, -50, 25, -45, 55, -25,
    70, 5, 65, 35, 40, 55, 5, 60, -25, 55,
    -55, 35, -65, 10, -65, -10,
  ] },
  // A second peak / hill to the east
  { points: [
    200, -160, 240, -180, 290, -170, 330, -130,
    350, -80, 340, -30, 300, 10, 250, 20,
    210, 0, 190, -40, 190, -90,
  ] },
  { points: [
    230, -130, 260, -145, 300, -130, 325, -95,
    335, -50, 320, -10, 290, 15, 255, 20,
    230, 5, 220, -30, 220, -70,
  ] },
  { points: [
    255, -95, 275, -105, 300, -95, 315, -65,
    320, -30, 310, 0, 285, 15, 265, 15,
    250, 0, 245, -30, 245, -60,
  ] },
  // A third area (southwest)
  { points: [
    -280, 140, -240, 120, -190, 130, -150, 170,
    -140, 220, -160, 260, -200, 280, -250, 260,
    -290, 220, -300, 180,
  ] },
  { points: [
    -250, 160, -220, 150, -190, 160, -170, 190,
    -165, 220, -180, 240, -210, 250, -240, 240,
    -260, 220, -265, 190,
  ] },
]

// ── Roads (paths connecting locations) ─────────────────────
export const roads: RoadData[] = [
  // Riverton → Darkwood
  { points: [50, 80, 120, 30, 200, -20, 300, -20] },
  // Riverton → Ancient Temple
  { points: [50, 80, -20, 120, -100, 130] },
  // Darkwood → Crystal Peaks
  { points: [300, -20, 350, -80, 380, -150] },
  // Ancient Temple → Abandoned Mine
  { points: [-100, 130, -180, 180, -250, 210] },
  // Riverton → Abandoned Mine (winding)
  { points: [50, 80, -30, 140, -120, 170, -200, 195, -250, 210] },
]

// ── Locations ──────────────────────────────────────────────
export const locations: LocationData[] = [
  { id: "riverton",  name: "Riverton",        x: 50,   y: 80,   type: "town" },
  { id: "darkwood",  name: "Darkwood",        x: 300,  y: -20,  type: "wilderness" },
  { id: "temple",    name: "Ancient Temple",  x: -100, y: 130,  type: "dungeon" },
  { id: "mine",      name: "Abandoned Mine",  x: -250, y: 210,  type: "dungeon" },
  { id: "peaks",     name: "Crystal Peaks",   x: 380,  y: -150, type: "landmark" },
  { id: "haven",     name: "Sunrise Haven",   x: -200, y: -150, type: "town" },
  { id: "outpost",   name: "Stone Outpost",   x: 180,  y: 180,  type: "landmark" },
]
