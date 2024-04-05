import { Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Note } from '@sp/graphql-types';

@Component({
  selector: 'app-note-ui',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './note-ui.component.html',
  styleUrl: './note-ui.component.scss',
})
export class NoteUiComponent {
  note = input.required<Note>();
}
