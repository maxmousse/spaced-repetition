import { Component, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Note } from '@sp/graphql-types';

@Component({
  selector: 'app-note-list-item-ui',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './note-list-item-ui.component.html',
  styleUrl: './note-list-item-ui.component.scss',
})
export class NoteListItemUiComponent {
  note = input.required<Pick<Note, 'title' | 'id'>>();
}
