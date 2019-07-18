import {Entity, PrimaryGeneratedColumn, Column, JoinColumn, OneToOne} from "typeorm";
import {User} from "./User";

@Entity()
export class UserWorkExperience {

  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  user: User;

  @Column()
  title: string;
  
  @Column()
  organization: string;

  @Column()
  location: string;
  
  @Column()
  fromDate: string;

  @Column()
  toDate: string;

  @Column()
  description: string;

  @Column()
  createdAt: Date;

  @Column()
  updatedAt: Date;  
}

