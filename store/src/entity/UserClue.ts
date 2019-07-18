import {Entity, PrimaryGeneratedColumn, Column, JoinColumn, OneToOne} from "typeorm";
import {User} from "./User";

@Entity()
export class UserClue {

  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  user: User;

  @Column()
  firstName: string;
  
  @Column()
  middleName: string;

  @Column()
  lastName: string;

  @Column()
  phoneNumber: string;

  @Column()
  mobileNumber: string;

  @Column()
  gender: string;

  @Column()
  birthDate: string;

  @Column()
  createdAt: Date;

  @Column()
  updatedAt: Date;  
}

