import {Entity, PrimaryGeneratedColumn, Column, JoinColumn, OneToOne} from "typeorm";
import {User} from "./User";

@Entity()
export class UserWorkPreference {

  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  user: User;
  
  @Column()
  interests: number;

  @Column()
  projectLimit: number;  
  
  @Column()
  projectLimitUpdatedAt: Date;

  @Column()
  createdAt: Date;

  @Column()
  updatedAt: Date;  
}
