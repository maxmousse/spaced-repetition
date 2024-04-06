import { Component, input } from '@angular/core';
import { toObservable } from '@angular/core/rxjs-interop';
import { CommonModule } from '@angular/common';
import { map, switchMap } from 'rxjs';
import { GetNoteQuery } from '../graphql/get-note.graphql';
import { NoteUiComponent } from '../note-ui/note-ui.component';

@Component({
  selector: 'app-note',
  standalone: true,
  imports: [CommonModule, NoteUiComponent],
  templateUrl: './note.component.html',
  styleUrl: './note.component.scss',
})
export class NoteComponent {
  noteId = input.required<string>();
  noteId$ = toObservable(this.noteId);
  note$ = this.noteId$.pipe(
    switchMap((id) => this.getNoteQuery.fetch({ input: { id } })),
    map((result) => result.data.getNote)
  );

  constructor(private getNoteQuery: GetNoteQuery) {}
}
