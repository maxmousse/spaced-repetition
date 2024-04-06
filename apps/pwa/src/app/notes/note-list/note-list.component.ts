import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GetNoteListQuery } from '../graphql/get-note-list.graphql';
import { map } from 'rxjs';
import { NoteListItemUiComponent } from '../note-list-item-ui/note-list-item-ui.component';

@Component({
  selector: 'app-note-list',
  standalone: true,
  imports: [CommonModule, NoteListItemUiComponent],
  templateUrl: './note-list.component.html',
  styleUrl: './note-list.component.scss',
})
export class NoteListComponent {
  notes$ = this.getNoteListQuery
    .fetch()
    .pipe(map((result) => result.data.getNotes));

  constructor(private getNoteListQuery: GetNoteListQuery) {}
}
