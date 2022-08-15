#include <cmath>
#include <cstdio>
#include <epoxy/gl.h>
#include <epoxy/glx.h>
#include <GL/freeglut.h>
#include <GLFW/glfw3.h> // GLFW is better than freeGLUT, but GLUT is oriented more towards quick/dirty like in tutorials
#include <stdexcept>

static const unsigned int screen_width = 160;
static const unsigned int screen_height = 120;
static const unsigned int pixelScale = 4;

struct timer
{
    int fr1, fr2;
};
timer T;

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


void draw_to_texture()
{
    // Create the framebuffer
    GLuint FramebufferName = 0;
    glGenFramebuffers(1, &FramebufferName);
    glBindFramebuffer(GL_FRAMEBUFFER, FramebufferName);

    // Create the texture
    GLuint renderedTexture;
    glGenTextures(1, &renderedTexture);
    // Bind the texture, all future texture functions will modify this texture
    glBindTexture(GL_TEXTURE_2D, renderedTexture);
    // Give an empty image
    glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB, screen_width, screen_height, 0, GL_RGB, GL_UNSIGNED_BYTE, 0);

    // Interpolation if scaling down
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
    // Interpolation if scaling up
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);

    // Attach the texture to the framebuffer, that is Set "renderedTexture" as our color attachment #0
    glFramebufferTexture(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, renderedTexture, 0);

    // Set the list of draw buffers
    GLenum DrawBuffers[1] = {GL_COLOR_ATTACHMENT0};
    glDrawBuffers(1, DrawBuffers);

    // Check if the framebuffer was successfully created
    if (glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE) 
        throw std::runtime_error("Framebuffer not initialized");
    
    // Bind the framebuffer
    glBindFramebuffer(GL_TEXTURE_2D, FramebufferName);
    glViewport(0, 0, screen_width, screen_height);

}

void display()
{
    if (T.fr1-T.fr2>=500) // timer in ms between frames
    {
        glClear(GL_COLOR_BUFFER_BIT);
        movePlayer();
        draw3D();

        T.fr2=T.fr1;
        glutSwapBuffers();
        glutReshapeWindow(screen_width<<2, screen_height<<2);
    }

    T.fr1=glutGet(GLUT_ELAPSED_TIME); // Elapsed timer in milliseconds
    glutPostRedisplay();
}

// Key press callbacks
static void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods)
{
    if (key==GLFW_KEY_W && action == GLFW_PRESS)      {K.w =1;}
    if (key==GLFW_KEY_S && action == GLFW_PRESS)      {K.s =1;}
    if (key==GLFW_KEY_A && action == GLFW_PRESS)      {K.a =1;}
    if (key==GLFW_KEY_D && action == GLFW_PRESS)      {K.d =1;}
    if (key==GLFW_KEY_M && action == GLFW_PRESS)      {K.m =1;}
    if (key==GLFW_KEY_COMMA && action == GLFW_PRESS)  {K.sr=1;}
    if (key==GLFW_KEY_PERIOD && action == GLFW_PRESS) {K.sl=1;}

    if (key==GLFW_KEY_W && action == GLFW_RELEASE)      {K.w =0;}
    if (key==GLFW_KEY_S && action == GLFW_RELEASE)      {K.s =0;}
    if (key==GLFW_KEY_A && action == GLFW_RELEASE)      {K.a =0;}
    if (key==GLFW_KEY_D && action == GLFW_RELEASE)      {K.d =0;}
    if (key==GLFW_KEY_M && action == GLFW_RELEASE)      {K.m =0;}
    if (key==GLFW_KEY_COMMA && action == GLFW_RELEASE)  {K.sr=0;}
    if (key==GLFW_KEY_PERIOD && action == GLFW_RELEASE) {K.sl=0;}
}

static void error_callback(int err, const char* description)
{
    fprintf(stderr, "Error: %s\n", description);
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
    // Select the context belonging to window
    glfwMakeContextCurrent(window);

    glClearColor(0.f, 60./255.f, 130./255.f, 0.f);

    

    gluOrtho2D(0, screen_width<<2, 0, screen_height<<2);
    glutDisplayFunc(display);

    glfwSetKeyCallback(window, key_callback);
    glfwSetErrorCallback(error_callback);

    // Main Loop
    while (!glfwWindowShouldClose(window))
    {
        glClear(GL_COLOR_BUFFER_BIT);

        glfwSwapBuffers(window);

        glfwPollEvents(); // Trigger callbacks
    }

    glfwTerminate();
    return 0;
}