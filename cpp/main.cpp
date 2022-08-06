#include <cmath>
#include <cstdio>
#include <epoxy/gl.h>
#include <epoxy/glx.h>
#include <GL/freeglut.h>
#include <GLFW/glfw3.h> // GLFW is better than freeGLUT, but GLUT is oriented more towards quick/dirty like in tutorials

static const unsigned int screen_width = 160;
static const unsigned int screen_height = 120;
static const unsigned int pixelScale = 4;

struct time
{
    int fr1, fr2;
};
time T;

struct keys
{
    int w,s,a,d;
    int sl,sr;
    int m;
};
keys K;

void pixel(int x, int y, int c)
{
    int rgb[3];
    if (c==0) {rgb[0]=255; rgb[1]=255; rgb[2]=  0;} //Yellow
    if (c==1) {rgb[0]=160; rgb[1]=160; rgb[2]=  0;} //Dark Yellow
    if (c==2) {rgb[0]=  0; rgb[1]=255; rgb[2]=  0;} //Green
    if (c==3) {rgb[0]=  0; rgb[1]=160; rgb[2]=  0;} //Dark Green
    if (c==4) {rgb[0]=  0; rgb[1]=255; rgb[2]=255;} //Cyan
    if (c==5) {rgb[0]=  0; rgb[1]=160; rgb[2]=160;} //Dark Cyan
    if (c==6) {rgb[0]=160; rgb[1]=100; rgb[2]=  0;} //Brown
    if (c==7) {rgb[0]=110; rgb[1]= 50; rgb[2]=  0;} //Dark Brown
    if (c==8) {rgb[0]=  0; rgb[1]= 60; rgb[2]=130;} //Background

    glColor3ub(rgb[0], rgb[1], rgb[2]);
    glBegin(GL_POINTS);
    glVertex2i(x*pixelScale+2, y*pixelScale+2);
    glEnd();
}

void movePlayer()
{
    if(K.a==1 && K.m==0) {printf("left\n");}
    if(K.d==1 && K.m==0) {printf("right\n");}
    if(K.w==1 && K.m==0) {printf("up\n");}
    if(K.s==1 && K.m==0) {printf("down\n");}

    if(K.a==1 && K.m==0) {printf("strafe left\n");}
    if(K.a==1 && K.m==0) {printf("strafe right\n");}

    if(K.a==1 && K.m==1) {printf("look left\n");}
    if(K.d==1 && K.m==1) {printf("look right\n");}
    if(K.w==1 && K.m==1) {printf("look up\n");}
    if(K.s==1 && K.m==1) {printf("look down\n");}
}


int tick;
void draw3D()
{
    int c = 0;
    for (size_t y = 0; y < screen_height>>1; y++)
    {
        for (size_t x = 0; x < screen_width>>1; x++)
        {
            pixel(x,y,c);
            c+=1;
            if (c>8) {c=0;}
        }
    }
    
    tick += 1;
    if (tick>20) {tick=0;}
    pixel(screen_width>>1, (screen_height>>1)+tick, 0);
}

void display()
{
    if (T.fr1-T.fr2>=500) // time in ms between frames
    {
        glClear(GL_COLOR_BUFFER_BIT);
        movePlayer();
        draw3D();

        T.fr2=T.fr1;
        glutSwapBuffers();
        glutReshapeWindow(screen_width<<2, screen_height<<2);
    }

    T.fr1=glutGet(GLUT_ELAPSED_TIME); // Elapsed time in milliseconds
    glutPostRedisplay();
}

// Key press callbacks
// What does chained == mean?
void KeysDown(unsigned char key, int x, int y)
{
    if (key=='w'==1) {K.w =1;}
    if (key=='s'==1) {K.s =1;}
    if (key=='a'==1) {K.a =1;}
    if (key=='d'==1) {K.d =1;}
    if (key=='m'==1) {K.m =1;}
    if (key==','==1) {K.sr=1;}
    if (key=='.'==1) {K.sl=1;}
}

void KeysUp(unsigned char key, int x, int y)
{
    if (key=='w'==1) {K.w =0;}
    if (key=='s'==1) {K.s =0;}
    if (key=='a'==1) {K.a =0;}
    if (key=='d'==1) {K.d =0;}
    if (key=='m'==1) {K.m =0;}
    if (key==','==1) {K.sr=0;}
    if (key=='.'==1) {K.sl=0;}
}

void init()
{
    
}

int main(int argcount, char* argvalues[])
{
    if (!glfwInit())
    {
        fprintf(stderr, "Failed to initialize GLFW\n");
        getchar();
        return -1;
    }

    GLFWwindow* window = glfwCreateWindow(screen_width<<2, screen_height<<2, "Window Title", NULL, NULL);
    if (window == NULL)
    {
        fprintf(stderr, "Failed to open GLFW window.");
        getchar();
        glfwTerminate();
        return -1;
    }

    glClearColor(0.f, 60./255.f, 130./255.f, 0.f);

    
    glutInit(&argcount, argvalues);
    gluOrtho2D(0, screen_width<<2, 0, screen_height<<2);
    init();
    glutDisplayFunc(display);
    glutKeyboardFunc(KeysDown);
    glutKeyboardUpFunc(KeysUp);
    glutMainLoop();

    glfwTerminate();
    return 0;
}