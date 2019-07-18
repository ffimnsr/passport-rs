import {Entity, PrimaryGeneratedColumn, Column} from "typeorm";

@Entity()
export class Industry {

  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  name: string;

  @Column()
  status: number;  
  
  @Column()
  createdAt: Date;

  @Column()
  updatedAt: Date;  
}
