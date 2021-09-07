import { Component, OnInit } from '@angular/core';
import { find_path } from '../../../pathfinder-wasm/pkg/pathfinder_wasm'

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'pathfinder-client';

  rows = 40;
  cols = 40;
  map: number[][] = Array.from(Array(this.rows), () => new Array(this.cols).fill(0));
  state = "start"
  start = [0, 0]
  end = [this.rows - 1, this.cols - 1];

  ngOnInit(): void {
    this.map[this.start[1]][this.start[0]] = 2;
    this.map[this.end[1]][this.end[0]] = 3;
  }

  onMapClick(x: number, y: number) {
    // alert(`${x} ${y}`)
    switch(this.state) {
      case 'start': {
        this.map[this.start[1]][this.start[0]] = 0;
        this.start = [x, y]
        this.map[y][x] = 2;
        break;
      }
      case 'finish': {
        this.map[this.end[1]][this.end[0]] = 0;
        this.end = [x, y]
        this.map[y][x] = 3;
        break;
      }
      case 'wall': {
        this.map[y][x] = 1;
        break;
      }
    }
    // this.map[y][x] = this.map[y][x] == 0 ? 1 : 0;
  }
  onFind() {
    const walls = new Array<{x: number; y: number;}>();
    for(let i = 0; i < this.rows; i++) {
      for(let j = 0; j < this.cols; j++) {
        if (this.map[j][i] == 2) {
          this.map[j][i] = 0;
        }
        if (this.map[j][i] == 1) {
          walls.push({x: i, y: j});
        }
      }
    }
    const path = find_path(this.start[0], this.start[1], this.end[0], this.end[1], 80, 40, 20, 1000, 0.05,
      Uint32Array.of(...walls.map(w => w.x)), Uint32Array.of(...walls.map(w => w.y)));

    console.log(path)
    let x = this.start[0], y = this.start[1];
    for (let v of path) {
      const dx = v == 0 ? -1 : v == 2 ? 1 : 0;
      const dy = v == 1 ? -1 : v == 3 ? 1 : 0;
      x += dx;
      y += dy;
      if (x < 0 || y < 0 || y >= this.rows || x >= this.cols || this.map[y][x] == 1) {
        x -= dx;
        y -= dy;
      }
      this.map[y][x] = 2;
    }
  }
}
