import {Entity, PrimaryGeneratedColumn, Column} from "typeorm";

@Entity()
export class User {

  @PrimaryGeneratedColumn()
  id: number;

  @Column()
  email: string;

  @Column()
  emailConfirmed: boolean;  
  
  @Column()
  passwordHash: string;

  @Column()
  createdAt: Date;

  @Column()
  updatedAt: Date;  
}
