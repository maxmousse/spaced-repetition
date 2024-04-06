import { ComponentFixture, TestBed } from '@angular/core/testing';
import { NoteListItemUiComponent } from './note-list-item-ui.component';

describe('NoteListItemUiComponent', () => {
  let component: NoteListItemUiComponent;
  let fixture: ComponentFixture<NoteListItemUiComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [NoteListItemUiComponent],
    }).compileComponents();

    fixture = TestBed.createComponent(NoteListItemUiComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
